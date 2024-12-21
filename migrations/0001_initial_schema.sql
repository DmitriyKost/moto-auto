begin
;

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

-- Сотрудник может работать в нескольких филиалах только в том случае,
-- если они расположены в одном городе, его данные должны быть доступны во всех
-- соответствующих авто и/или мото сервисах.
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
    bonus_points INTEGER NOT NULL DEFAULT 0 ,
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
    status VARCHAR(20) NOT NULL CHECK (Status IN ('confirmed', 'pending', 'cancelled'))
);

CREATE OR REPLACE FUNCTION calculate_total_amount()
RETURNS TRIGGER AS $$
DECLARE
    service_total NUMERIC(15, 2) := 0;
    part_total NUMERIC(15, 2) := 0;
BEGIN
    SELECT COALESCE(SUM(sb.price), 0) INTO service_total
    FROM moto_auto.order_service os
    INNER JOIN moto_auto.service_branch sb
    ON os.service_id = sb.service_id
    WHERE os.order_id = NEW.order_id;

    SELECT COALESCE(SUM(spb.price * osp.quantity), 0) INTO part_total
    FROM moto_auto.order_service_part osp
    INNER JOIN moto_auto.spare_part_branch spb
    ON osp.part_id = spb.part_id
    INNER JOIN moto_auto.order_service os
    ON osp.order_service_id = os.order_service_id
    WHERE os.order_id = NEW.order_id;

    UPDATE moto_auto.orders
    SET total_amount = service_total + part_total
    WHERE order_id = NEW.order_id;

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
-- Статус клиента определяется автоматически по количеству потраченных на услуги
-- сервиса денег.
create or replace function update_client_status()
returns trigger
as $$
DECLARE
    total_spent NUMERIC(15, 2);
BEGIN
    SELECT SUM(total_amount) INTO total_spent
    FROM moto_auto.orders
    WHERE client_id = NEW.client_id;

    UPDATE moto_auto.client
    SET client.total_spent = total_spent,
        client.status = CASE
                    WHEN total_spent < 10000 THEN 'casual'
                    WHEN total_spent >= 10000 AND total_spent < 50000 THEN 'regular'
                    ELSE 'premium'
                 END
    WHERE client_id = NEW.client_id;

    RETURN NEW;
END;
$$
language plpgsql
;

CREATE TRIGGER trigger_update_client_status
AFTER INSERT OR UPDATE ON moto_auto.orders
FOR EACH ROW
EXECUTE FUNCTION update_client_status();

commit
;
