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
    employee_id INTEGER REFERENCES moto_auto.employee(employee_id) ON DELETE CASCADE,
    branch_id INTEGER REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE
);

CREATE TABLE moto_auto.client (
    client_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    contact_info TEXT NOT NULL,
    status VARCHAR(20) CHECK (status IN ('casual', 'regular', 'premium')),
    bonus_points INTEGER NOT NULL DEFAULT 0 ,
    total_spent NUMERIC(15, 2) NOT NULL DEFAULT 0 
);

CREATE TABLE moto_auto."order" (
    order_id SERIAL PRIMARY KEY,
    client_id INTEGER REFERENCES moto_auto.client(client_id),
    branch_id INTEGER REFERENCES moto_auto.branch(branch_id),
    order_date TIMESTAMPTZ NOT NULL DEFAULT NOW() ,
    completion_date TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) CHECK (status IN ('processing', 'finished', 'cancelled'))
);

CREATE TABLE moto_auto.service (
    service_id SERIAL PRIMARY KEY,
    service_name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE moto_auto.service_branch (
    service_branch_id SERIAL PRIMARY KEY,
    price NUMERIC(15, 2) NOT NULL,
    branch_id INTEGER REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE
);

CREATE TABLE moto_auto.spare_part (
    part_id SERIAL PRIMARY KEY,
    part_name VARCHAR(100) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE moto_auto.spare_part_branch (
    spare_part_branch_id SERIAL PRIMARY KEY,
    part_id INTEGER REFERENCES moto_auto.spare_part(part_id) ON DELETE CASCADE,
    branch_id INTEGER REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE,
    stock_quantity INTEGER NOT NULL DEFAULT 0,
    price NUMERIC(15, 2) NOT NULL
);

CREATE TABLE moto_auto.order_service (
    order_service_id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES moto_auto."order"(order_id),
    service_id INTEGER REFERENCES moto_auto.service(service_id)
);

CREATE TABLE moto_auto.order_service_part (
    order_service_part_id SERIAL PRIMARY KEY,
    part_id INTEGER REFERENCES moto_auto.spare_part(part_id),
    quantity INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE moto_auto."user" (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    passwordhash VARCHAR(255) NOT NULL,
    role VARCHAR(20) CHECK (role IN ('admin', 'analyst', 'master', 'manager')),
    branch_id INTEGER REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE
);

CREATE TABLE moto_auto.schedule (
    schedule_id SERIAL PRIMARY KEY,
    client_id INTEGER REFERENCES moto_auto.client(client_id) ON DELETE CASCADE,
    branch_id INTEGER REFERENCES moto_auto.branch(branch_id) ON DELETE CASCADE,
    service_id INTEGER REFERENCES moto_auto.service(service_id),
    preffered_master_id INTEGER REFERENCES moto_auto.employee(employee_id),
    scheduled_datetime TIMESTAMP NOT NULL,
    status VARCHAR(20) CHECK (Status IN ('confirmed', 'cancelled'))
);

-- Статус клиента определяется автоматически по количеству потраченных на услуги
-- сервиса денег.
create or replace function update_client_status()
returns trigger
as $$
DECLARE
    total_spent NUMERIC(15, 2);
BEGIN
    SELECT SUM(total_amount) INTO total_spent
    FROM moto_auto."order"
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
AFTER INSERT OR UPDATE ON moto_auto."order"
FOR EACH ROW
EXECUTE FUNCTION update_client_status();

commit
;
