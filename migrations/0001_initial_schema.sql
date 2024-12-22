BEGIN;

CREATE SCHEMA moto_auto;

CREATE TABLE moto_auto.branch (
    branch_id SERIAL PRIMARY KEY,
    address VARCHAR(255) NOT NULL,
    phone_number VARCHAR(15) NOT NULL,
    postal_code VARCHAR(10) NOT NULL,
    employee_count INTEGER NOT NULL,
    city VARCHAR(100) NOT NULL
);

CREATE TABLE moto_auto.users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    passwordhash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL CHECK (role IN ('admin', 'analyst', 'master', 'manager')),
    branch_id INTEGER NOT NULL REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE
);

CREATE TABLE moto_auto.employee (
    employee_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    age INTEGER NOT NULL,
    position VARCHAR(50) NOT NULL,
    contact_info TEXT NOT NULL,
    expirience_years INTEGER NOT NULL,
    salary NUMERIC(15, 2) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE moto_auto.branch_employee (
    branch_employee_id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL REFERENCES moto_auto.employee(employee_id) ON DELETE CASCADE,
    branch_id INTEGER NOT NULL REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE
);

CREATE TABLE moto_auto.client (
    client_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    contact_info TEXT NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('casual', 'regular', 'premium')),
    bonus_points NUMERIC(15, 2) DEFAULT 0,
    total_spent NUMERIC(15, 2) NOT NULL DEFAULT 0 
);

CREATE TABLE moto_auto.orders (
    order_id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES moto_auto.client(client_id),
    branch_id INTEGER NOT NULL REFERENCES moto_auto.branch(branch_id),
    master_id INTEGER NOT NULL REFERENCES moto_auto.users(user_id),
    order_date TIMESTAMPTZ NOT NULL DEFAULT NOW() ,
    completion_date TIMESTAMPTZ,
    total_amount NUMERIC(15, 2),
    status VARCHAR(20) NOT NULL CHECK (status IN ('processing', 'finished', 'cancelled'))
);

CREATE TABLE moto_auto.service (
    service_id SERIAL PRIMARY KEY,
    service_name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE moto_auto.service_branch (
    service_branch_id SERIAL PRIMARY KEY,
    price NUMERIC(15, 2) NOT NULL,
    branch_id INTEGER NOT NULL REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE,
    service_id INTEGER NOT NULL REFERENCES moto_auto.service(service_id) ON DELETE CASCADE
);

CREATE TABLE moto_auto.spare_part (
    part_id SERIAL PRIMARY KEY,
    part_name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE moto_auto.spare_part_branch (
    spare_part_branch_id SERIAL PRIMARY KEY,
    part_id INTEGER NOT NULL REFERENCES moto_auto.spare_part(part_id) ON DELETE CASCADE,
    branch_id INTEGER NOT NULL REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE,
    stock_quantity INTEGER NOT NULL DEFAULT 0,
    price NUMERIC(15, 2) NOT NULL
);

CREATE TABLE moto_auto.order_service (
    order_service_id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES moto_auto.orders(order_id),
    service_id INTEGER NOT NULL REFERENCES moto_auto.service(service_id)
);

CREATE TABLE moto_auto.order_service_part (
    order_service_part_id SERIAL PRIMARY KEY,
    part_id INTEGER NOT NULL REFERENCES moto_auto.spare_part(part_id),
    order_service_id INTEGER NOT NULL REFERENCES moto_auto.order_service(order_service_id),
    quantity INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE moto_auto.schedule (
    schedule_id SERIAL PRIMARY KEY,
    client_id INTEGER NOT NULL REFERENCES moto_auto.client(client_id) ON DELETE CASCADE,
    branch_id INTEGER NOT NULL REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE,
    order_id INTEGER NOT NULL REFERENCES moto_auto.orders(order_id),
    scheduled_datetime TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('confirmed', 'pending', 'cancelled'))
);

CREATE OR REPLACE FUNCTION calculate_total_amount()
RETURNS TRIGGER AS $$
DECLARE
    service_total NUMERIC(15, 2) := 0;
    part_total NUMERIC(15, 2) := 0;
    current_order_id INTEGER;
BEGIN
    IF TG_TABLE_NAME = 'order_service' THEN
        SELECT os.order_id INTO current_order_id
        FROM moto_auto.order_service os
        WHERE os.order_service_id = NEW.order_service_id;
    ELSIF TG_TABLE_NAME = 'order_service_part' THEN
        SELECT os.order_id INTO current_order_id
        FROM moto_auto.order_service os
        WHERE os.order_service_id = NEW.order_service_id;
    END IF;

    SELECT COALESCE(SUM(sb.price), 0) INTO service_total
    FROM moto_auto.order_service os
    INNER JOIN moto_auto.service_branch sb
    ON os.service_id = sb.service_id
    WHERE os.order_id = current_order_id;

    SELECT COALESCE(SUM(spb.price * osp.quantity), 0) INTO part_total
    FROM moto_auto.order_service_part osp
    INNER JOIN moto_auto.spare_part_branch spb
    ON osp.part_id = spb.part_id
    INNER JOIN moto_auto.order_service os
    ON osp.order_service_id = os.order_service_id
    WHERE os.order_id = current_order_id;

    UPDATE moto_auto.orders
    SET total_amount = service_total + part_total
    WHERE order_id = current_order_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_calculate_total_amount_order_service
AFTER INSERT OR UPDATE ON moto_auto.order_service
FOR EACH ROW
EXECUTE FUNCTION calculate_total_amount();

CREATE TRIGGER trigger_calculate_total_amount_order_service_part
AFTER INSERT OR UPDATE ON moto_auto.order_service_part
FOR EACH ROW
EXECUTE FUNCTION calculate_total_amount();


CREATE OR REPLACE FUNCTION update_client_status()
RETURNS TRIGGER
AS $$
DECLARE
    client_total_spent NUMERIC(15, 2);
BEGIN
    SELECT SUM(total_amount) INTO client_total_spent
    FROM moto_auto.orders
    WHERE client_id = NEW.client_id;

    UPDATE moto_auto.client
    SET total_spent = client_total_spent,
        status = CASE
                    WHEN client_total_spent < 10000 THEN 'casual'
                    WHEN client_total_spent >= 10000 AND client_total_spent < 50000 THEN 'regular'
                    ELSE 'premium'
                 END
    WHERE client_id = NEW.client_id;

    RETURN NEW;
END;
$$
LANGUAGE plpgsql;
CREATE TRIGGER trigger_update_client_status
AFTER INSERT OR UPDATE ON moto_auto.orders
FOR EACH ROW
WHEN (NEW.status = 'finished') 
EXECUTE FUNCTION update_client_status();

CREATE OR REPLACE FUNCTION add_bonus_points_by_status()
RETURNS TRIGGER AS $$
DECLARE
    bonus_multiplier NUMERIC := 0;
    calculated_bonus_points INTEGER;
BEGIN
    IF (NEW.status = 'finished' AND (TG_OP = 'INSERT' OR OLD.status != 'finished')) THEN
        SELECT CASE
            WHEN c.status = 'casual' THEN 0.1
            WHEN c.status = 'regular' THEN 0.2
            WHEN c.status = 'premium' THEN 0.3
        END INTO bonus_multiplier
        FROM moto_auto.client c
        WHERE c.client_id = NEW.client_id;

        calculated_bonus_points := FLOOR(NEW.total_amount * bonus_multiplier);

        UPDATE moto_auto.client
        SET bonus_points = COALESCE(bonus_points, 0) + calculated_bonus_points
        WHERE client_id = NEW.client_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_add_bonus_points_by_status
AFTER INSERT OR UPDATE ON moto_auto.orders
FOR EACH ROW
WHEN (NEW.status = 'finished') 
EXECUTE FUNCTION add_bonus_points_by_status();

CREATE OR REPLACE FUNCTION increment_employee_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' AND OLD.branch_id IS DISTINCT FROM NEW.branch_id THEN
        UPDATE moto_auto.branch
        SET employee_count = employee_count - 1
        WHERE branch_id = OLD.branch_id;
    END IF;

    UPDATE moto_auto.branch
    SET employee_count = employee_count + 1
    WHERE branch_id = NEW.branch_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_increment_employee_count
AFTER INSERT OR UPDATE ON moto_auto.branch_employee
FOR EACH ROW
EXECUTE FUNCTION increment_employee_count();

CREATE OR REPLACE FUNCTION expire_bonus_points()
RETURNS VOID AS $$
BEGIN
    IF EXTRACT(MONTH FROM CURRENT_DATE) = 1 AND EXTRACT(DAY FROM CURRENT_DATE) = 1 THEN
        UPDATE moto_auto.client
        SET bonus_points = 0;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- ROLES
CREATE ROLE admin;
CREATE ROLE analyst;
CREATE ROLE master;
CREATE ROLE manager;


-- Администраторы имеют полный доступ к системе и могут управлять пользователями
GRANT ALL PRIVILEGES ON SCHEMA moto_auto TO admin;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA moto_auto TO admin;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA moto_auto TO admin;
GRANT CREATE ON DATABASE moto_auto TO admin;

-- Аналитики имеют доступ на чтение из любой таблицы.
GRANT USAGE ON SCHEMA moto_auto TO analyst;
GRANT SELECT ON ALL TABLES IN SCHEMA moto_auto TO analyst;
ALTER DEFAULT PRIVILEGES IN SCHEMA moto_auto GRANT SELECT ON TABLES TO analyst;

-- Мастера могут просматривать информацию о заказах, клиентах, оборудовании и запчастях, а также вносить данные о выполненной работе.
GRANT USAGE ON SCHEMA moto_auto TO master;
GRANT SELECT ON moto_auto.orders TO master;
GRANT SELECT ON moto_auto.client TO master;
GRANT SELECT ON moto_auto.spare_part TO master;
GRANT SELECT ON moto_auto.spare_part_branch TO master;

GRANT UPDATE, INSERT ON moto_auto.orders TO master;

-- В целях безопасности, мастера могут видеть в информационной системе только тех клиентов, с которыми они работают.
ALTER TABLE moto_auto.client ENABLE ROW LEVEL SECURITY;
CREATE POLICY master_client_policy ON moto_auto.client
    FOR SELECT TO master USING (
        EXISTS (
            SELECT 1
            FROM moto_auto.orders
            WHERE moto_auto.orders.client_id = moto_auto.client.client_id
              AND moto_auto.orders.master_id = (
                  SELECT user_id
                  FROM moto_auto.users
                  WHERE username = current_user AND role = 'master'
              )
        )
    );

ALTER TABLE moto_auto.orders ENABLE ROW LEVEL SECURITY;
CREATE POLICY master_orders_policy ON moto_auto.orders
    FOR ALL TO master USING (
        moto_auto.orders.master_id = (
            SELECT user_id
            FROM moto_auto.users
            WHERE username = current_user AND role = 'master'
        )
    );

-- Менеджеры имеют доступ к информации о клиентах, заказах и чеках, могут добавлять новых клиентов и управлять заказами того автосервиса, в котором числятся менеджерами. 
CREATE POLICY manager_client_policy ON moto_auto.client
    FOR ALL TO manager USING (
        EXISTS (
            SELECT 1
            FROM moto_auto.users
            WHERE moto_auto.users.branch_id = (
                SELECT branch_id
                FROM moto_auto.users
                WHERE username = current_user AND role = 'manager'
            )
        )
    );

CREATE POLICY manager_orders_policy ON moto_auto.orders
    FOR ALL TO manager USING (
        moto_auto.orders.branch_id = (
            SELECT branch_id
            FROM moto_auto.users
            WHERE username = current_user AND role = 'manager'
        )
    );

-- Администраторы имеют полный доступ к системе, того автосервиса, где числятся администраторами.
ALTER TABLE moto_auto.users ENABLE ROW LEVEL SECURITY;
CREATE POLICY admin_user_policy ON moto_auto.users
    FOR ALL TO admin USING (
        moto_auto.users.branch_id = (
            SELECT branch_id
            FROM moto_auto.users
            WHERE username = current_user AND role = 'admin'
        )
    );

-- Индексы для заказов
CREATE INDEX idx_orders_client_id ON moto_auto.orders(client_id);
CREATE INDEX idx_orders_branch_id ON moto_auto.orders(branch_id);
CREATE INDEX idx_orders_master_id ON moto_auto.orders(master_id);
CREATE INDEX idx_orders_status ON moto_auto.orders(status);
CREATE INDEX idx_orders_order_date ON moto_auto.orders(order_date);

CREATE INDEX idx_order_service_order_id ON moto_auto.order_service(order_id);
CREATE INDEX idx_order_service_service_id ON moto_auto.order_service(service_id);

-- Индексы для расписания
CREATE INDEX idx_schedule_client_id ON moto_auto.schedule(client_id);
CREATE INDEX idx_schedule_branch_id ON moto_auto.schedule(branch_id);
CREATE INDEX idx_schedule_order_id ON moto_auto.schedule(order_id);
CREATE INDEX idx_schedule_status ON moto_auto.schedule(status);

-- TEST (uncomment if u wanna test data)
INSERT INTO moto_auto.branch (address, phone_number, postal_code, employee_count, city)
VALUES 
('123 Main St', '123-456-7890', '12345', 10, 'New York'),
('456 Elm St', '123-456-7891', '23456', 8, 'Los Angeles'),
('789 Pine St', '123-456-7892', '34567', 5, 'Chicago');

INSERT INTO moto_auto.users (username, passwordhash, role, branch_id)
VALUES
('admin1', 'b56a96a2daa2d0e13bfed0ab5ed2e56fd2152682e8b078515e6058f0f301a059', 'admin', 1),
('analyst1', 'fb82425cada0f96011ed8e2cdf25679de1168752877d636858dac6b6eb2c7559', 'analyst', 1),
('manager1', 'b1ba652e4797e6d49d92c0a48dfaaf2ad4168e3ebc028218cab7a959f0985472', 'manager', 1),
('master1', '68156596aacb0217cba4ee279dfcb95a55eec84c0dcb24ec5d17f97e860b48e1', 'master', 1);

INSERT INTO moto_auto.employee (name, age, position, contact_info, expirience_years, salary, description)
VALUES
('John Doe', 35, 'Mechanic', 'john.doe@example.com', 10, 55000, 'Skilled in motorcycle repairs'),
('Jane Smith', 28, 'Technician', 'jane.smith@example.com', 5, 42000, 'Specializes in diagnostics'),
('Michael Johnson', 40, 'Manager', 'michael.johnson@example.com', 15, 70000, 'Experienced team leader'),
('Emily White', 32, 'Customer Service', 'emily.white@example.com', 8, 45000, 'Great at handling clients');

INSERT INTO moto_auto.branch_employee (employee_id, branch_id)
VALUES
(1, 1),
(2, 1),
(3, 2),
(4, 3);

INSERT INTO moto_auto.client (name, contact_info, status, bonus_points, total_spent)
VALUES
('Alice Cooper', 'alice.cooper@example.com', 'casual', 0, 0),
('Bob Marley', 'bob.marley@example.com', 'casual', 0, 0),
('Charlie Brown', 'charlie.brown@example.com', 'casual', 0, 0);

INSERT INTO moto_auto.service (service_name, description) VALUES
('Oil Change', 'Change of oil for motorcycles'),
('Brake Repair', 'Replacing or repairing the brake system'),
('Tire Replacement', 'Changing worn-out tires on bikes');

INSERT INTO moto_auto.service_branch (price, branch_id, service_id)
VALUES
(50, 1, 1),
(50, 2, 1),
(50, 3, 1),
(75, 1, 2),
(75, 2, 2),
(75, 3, 2),
(100, 1, 3),
(100, 2, 3),
(100, 3, 3);

INSERT INTO moto_auto.spare_part (part_name, description)
VALUES
('Brake Pads', 'High-quality brake pads for motorcycles'),
('Motor Oil', 'Synthetic oil for engine lubrication'),
('Tires', 'Rubber tires for motorcycles');

INSERT INTO moto_auto.spare_part_branch (part_id, branch_id, stock_quantity, price)
VALUES
(1, 1, 20, 30),
(2, 1, 20, 30),
(3, 1, 20, 30),
(1, 2, 20, 30),
(2, 2, 20, 30),
(3, 2, 20, 30),
(1, 3, 20, 30),
(2, 3, 15, 40),
(3, 3, 10, 50);

DO $$
DECLARE
    i INTEGER;
BEGIN
    FOR i IN 1..5000 LOOP
        INSERT INTO moto_auto.orders (client_id, branch_id, master_id, total_amount, status)
        VALUES
            (FLOOR(1 + RANDOM() * 3), FLOOR(1 + RANDOM() * 3), 4, NULL, 'processing');
    END LOOP;
END;
$$;

DO $$
DECLARE
    i INTEGER;
BEGIN
    FOR i IN 1..5000 LOOP
        INSERT INTO moto_auto.order_service (order_id, service_id)
        VALUES
            (FLOOR(1+RANDOM() * 5000), FLOOR(1 + RANDOM() * 3));
    END LOOP;
END;
$$;

DO $$
DECLARE
    i INTEGER;
BEGIN
    FOR i IN 1..5000 LOOP
        INSERT INTO moto_auto.order_service_part (order_service_id, part_id, quantity)
        VALUES
            (FLOOR(1 + RANDOM() * 5000), FLOOR(1 + RANDOM() * 3), 1);
    END LOOP;
END;
$$;

DO $$
DECLARE
    i INTEGER;
BEGIN
    FOR i IN 1..5000 LOOP
        UPDATE moto_auto.orders
        SET status = 'finished',
        completion_date = NOW()
        WHERE order_id = i
        AND total_amount IS NOT NULL;
    END LOOP;
END;
$$;

INSERT INTO moto_auto.schedule (client_id, branch_id, order_id, scheduled_datetime, status)
VALUES
(1, 1, 1, '2024-12-25 10:00:00', 'confirmed'),
(2, 2, 2, '2024-12-26 11:00:00', 'pending'),
(3, 3, 3, '2024-12-27 12:00:00', 'cancelled');

CREATE USER analyst1 PASSWORD 'password1';
GRANT analyst TO analyst1;
-- TEST
COMMIT;
