BEGIN;

INSERT INTO moto_auto.branch (address, phone_number, postal_code, employee_count, city) VALUES
('123 Main St', '1234567890', '10001', 10, 'New York'),
('456 Elm St', '9876543210', '20002', 15, 'Los Angeles'),
('789 Oak St', '5556667777', '30003', 8, 'Chicago'),
('101 Maple St', '3334445555', '40004', 12, 'Houston'),
('202 Pine St', '1112223333', '50005', 5, 'Phoenix');

INSERT INTO moto_auto.users (username, passwordhash, role, branch_id) VALUES
('admin1', 'hash1', 'admin', 1),
('manager1', 'hash2', 'manager', 1),
('analyst1', 'hash3', 'analyst', 2),
('master1', 'hash4', 'master', 2),
('master2', 'hash5', 'master', 3),
('manager2', 'hash6', 'manager', 3),
('admin2', 'hash7', 'admin', 4),
('analyst2', 'hash8', 'analyst', 4),
('master3', 'hash9', 'master', 5),
('manager3', 'hash10', 'manager', 5);

INSERT INTO moto_auto.employee (name, age, position, contact_info, expirience_years, salary, description) VALUES
('John Doe', 30, 'Technician', 'john@example.com', 5, 50000, 'Experienced in engine repair'),
('Jane Smith', 28, 'Technician', 'jane@example.com', 3, 45000, 'Expert in electrical systems'),
('Mike Johnson', 35, 'Manager', 'mike@example.com', 10, 60000, 'Branch manager'),
('Emily Davis', 26, 'Receptionist', 'emily@example.com', 2, 30000, 'Handles client bookings'),
('Robert Brown', 40, 'Technician', 'robert@example.com', 15, 55000, 'Specializes in transmissions'),

INSERT INTO moto_auto.branch_employee (employee_id, branch_id) VALUES
(1, 1),
(2, 1),
(3, 2),
(4, 2),
(5, 3),

INSERT INTO moto_auto.client (name, contact_info, status, bonus_points, total_spent) VALUES
('Alice Johnson', 'alice@example.com', 'casual', 0, 0),
('Bob Williams', 'bob@example.com', 'casual', 0, 0),
('Charlie Brown', 'charlie@example.com', 'casual', 0, 0),
('Diana Evans', 'diana@example.com', 'casual', 0, 0),

INSERT INTO moto_auto.orders (client_id, branch_id, master_id, order_date, completion_date, total_amount, status) VALUES
(1, 1, 4, '2023-12-01', '2023-12-02', 300, 'finished'),
(2, 2, 3, '2023-12-10', NULL, NULL, 'processing'),

INSERT INTO moto_auto.service (service_name, description) VALUES
('Wheel Alignment', 'Precise wheel alignment to improve car performance'),
('Brake Inspection', 'Thorough inspection of brake system'),
('Transmission Repair', 'Repair and maintenance of car transmission'),
('Engine Diagnostics', 'Comprehensive engine check-up'),
('AC Repair', 'Fix and maintain air conditioning system'),

INSERT INTO moto_auto.service_branch (price, branch_id, service_id) VALUES
(70, 1, 4),
(40, 1, 5),
(300, 1, 6),
(200, 2, 4),
(50, 2, 5),
(350, 2, 6),
(90, 3, 7),
(120, 3, 8),
(150, 3, 9),
(100, 4, 4),
(45, 4, 5),
(250, 4, 6),
(130, 5, 7),
(140, 5, 8),
(160, 5, 9),

INSERT INTO moto_auto.spare_part (part_name, description) VALUES
('Oil Filter', 'High-performance oil filter'),
('Car Battery', 'Long-lasting car battery'),
('Timing Belt', 'Durable timing belt'),
('Headlight Bulb', 'Bright and energy-efficient headlight bulb'),
('Windshield Wiper', 'High-quality windshield wiper'),
('Radiator Hose', 'Durable radiator hose'),

INSERT INTO moto_auto.spare_part_branch (part_id, branch_id, stock_quantity, price) VALUES
(4, 1, 10, 15),
(5, 1, 20, 100),
(6, 2, 12, 30),
(7, 2, 25, 50),
(8, 3, 40, 25),
(9, 3, 30, 20),
(10, 4, 35, 10),
(11, 5, 15, 75),
(12, 5, 10, 120),

INSERT INTO moto_auto.order_service (order_id, service_id) VALUES
(3, 4),
(3, 5),
(4, 6),
(5, 7),
(5, 8),
(6, 9),
(7, 1),
(7, 4),
(8, 3),
(8, 6),

INSERT INTO moto_auto.order_service_part (part_id, order_service_id, quantity) VALUES
(3, 2, 2),
(4, 2, 1),
(5, 3, 1),
(6, 4, 2),
(7, 5, 3),
(8, 6, 2),
(9, 7, 1),
(10, 8, 1),
(11, 9, 4),
(12, 10, 2),

INSERT INTO moto_auto.schedule (client_id, branch_id, order_id, scheduled_datetime, status) VALUES
(1, 1, 1, '2023-12-01 09:00:00', 'confirmed'),
(2, 2, 2, '2023-12-10 10:00:00', 'pending'),

DO $$
DECLARE
    branch_count INTEGER;
    client_count INTEGER;
    service_branch_count INTEGER;
    order_id INTEGER;
    i INTEGER := 1;
    random_branch_id INTEGER;
    random_client_id INTEGER;
    random_master_id INTEGER;
    random_service_branch RECORD;
BEGIN
    SELECT COUNT(*) INTO branch_count FROM moto_auto.branch;
    SELECT COUNT(*) INTO client_count FROM moto_auto.client;
    SELECT COUNT(*) INTO service_branch_count FROM moto_auto.service_branch;

    WHILE i <= 5000 LOOP
        random_branch_id := CEIL(RANDOM() * branch_count);
        random_client_id := CEIL(RANDOM() * client_count);
        SELECT user_id INTO random_master_id
        FROM moto_auto.users
        WHERE branch_id = random_branch_id AND role = 'master'
        ORDER BY RANDOM() LIMIT 1;

        -- Создаем заказ
        INSERT INTO moto_auto.orders (client_id, branch_id, master_id, order_date, status)
        VALUES (
            random_client_id,
            random_branch_id,
            random_master_id,
            NOW() - INTERVAL '1 DAY' * (RANDOM() * 365), 
            CASE
                WHEN RANDOM() < 0.7 THEN 'finished'
                WHEN RANDOM() < 0.9 THEN 'processing'
                ELSE 'cancelled'
            END
        )
        RETURNING order_id INTO order_id;

        FOR random_service_branch IN
            SELECT service_id, price
            FROM moto_auto.service_branch
            WHERE branch_id = random_branch_id
            ORDER BY RANDOM()
            LIMIT CEIL(RANDOM() * 3) 
        LOOP
            INSERT INTO moto_auto.order_service (order_id, service_id)
            VALUES (order_id, random_service_branch.service_id);
        END LOOP;

        i := i + 1;
    END LOOP;
END $$;
COMMIT;
