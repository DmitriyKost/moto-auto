CREATE TABLE branch (
    branch_id SERIAL PRIMARY KEY,
    address VARCHAR(255) NOT NULL,
    phone_number VARCHAR(15),
    postal_code VARCHAR(10),
    employee_count INTEGER,
    city VARCHAR(100)
);

CREATE TABLE employee (
    employee_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    age INTEGER,
    position VARCHAR(50),
    contact_info VARCHAR(100),
    expirience_years INTEGER,
    salary NUMERIC(10, 2),
    description TEXT,
);

-- Сотрудник может работать в нескольких филиалах только в том случае,
-- если они расположены в одном городе, его данные должны быть доступны во всех соответствующих авто и/или мото сервисах.
CREATE TABLE branch_employee (
    branch_employee_id SERIAL PRIMARY KEY,
    employee_id INTEGER REFERENCES employee(employee_id) ON DELETE CASCADE,
    branch_id INTEGER REFERENCES branch(branch_id) ON DELETE CASCADE
);


CREATE TABLE client (
    client_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    contact_info VARCHAR(255),
    status VARCHAR(20) CHECK (status IN ('Обычный', 'Постоянный', 'Премиум')),
    bonus_points INTEGER DEFAULT 0,
    total_spent NUMERIC(10, 2) DEFAULT 0
);

CREATE TABLE "order" (
    order_id SERIAL PRIMARY KEY,
    client_id INTEGER REFERENCES client(client_id) ON DELETE CASCADE,
    branch_id INTEGER REFERENCES branch(branch_id) ON DELETE CASCADE,
    order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completion_date TIMESTAMP,
    total_amount NUMERIC(10, 2),
    status VARCHAR(20) CHECK (status IN ('В процессе', 'Завершен', 'Отменен'))
);

-- Статус клиента определяется автоматически по количеству потраченных на услуги сервиса денег.
CREATE OR REPLACE FUNCTION update_client_status()
RETURNS TRIGGER AS $$
DECLARE
    total_spent NUMERIC(10, 2);
BEGIN
    SELECT SUM(total_amount) INTO total_spent
    FROM "order"
    WHERE client_id = NEW.client_id;

    UPDATE client
    SET client.total_spent = total_spent,
        client.status = CASE
                    WHEN total_spent < 10000 THEN 'Обычный'
                    WHEN total_spent >= 10000 AND total_spent < 50000 THEN 'Постоянный'
                    ELSE 'Премиум'
                 END
    WHERE client_id = NEW.client_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_client_status
AFTER INSERT OR UPDATE ON "order"
FOR EACH ROW
EXECUTE FUNCTION update_client_status();

CREATE TABLE service (
    service_id SERIAL PRIMARY KEY,
    service_name VARCHAR(100) NOT NULL,
    description TEXT,
    price NUMERIC(10, 2),
    branch_id INTEGER REFERENCES branch(branch_id) ON DELETE CASCADE
);

CREATE TABLE spare_part (
    part_id SERIAL PRIMARY KEY,
    part_name VARCHAR(100) NOT NULL,
    description TEXT,
    stock_quantity INTEGER DEFAULT 0,
    price NUMERIC(10, 2),
    branch_id INTEGER REFERENCES branch(branch_id) ON DELETE CASCADE
);

CREATE TABLE order_detail (
    order_detail_id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES "order"(order_id) ON DELETE CASCADE,
    service_id INTEGER REFERENCES service(service_id),
    part_id INTEGER REFERENCES spare_part(part_id),
    quantity INTEGER DEFAULT 1,
    sub_total NUMERIC(10, 2)
);

CREATE TABLE "user" (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    passwordhash VARCHAR(255) NOT NULL,
    role VARCHAR(20) CHECK (role IN ('Администратор', 'Аналитик', 'Мастер', 'Менеджер')),
    branch_id INTEGER REFERENCES branch(branch_id)
);

CREATE TABLE schedule (
    schedule_id SERIAL PRIMARY KEY,
    client_id INTEGER REFERENCES client(client_id) ON DELETE CASCADE,
    branch_id INTEGER REFERENCES branch_id(branch_id) ON DELETE CASCADE,
    service_id INTEGER REFERENCES service(service_id),
    preffered_master_id INTEGER REFERENCES employee(employee_id),
    scheduled_datetime TIMESTAMP NOT NULL,
    status VARCHAR(20) CHECK (Status IN ('Подтверждено', 'Отменено'))
);
