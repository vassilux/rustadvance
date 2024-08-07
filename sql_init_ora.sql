-- Create the user
CREATE USER rustUser IDENTIFIED BY rustPassword;

-- Grant the necessary privileges
GRANT CONNECT, RESOURCE TO rustUser;
ALTER USER rustUser QUOTA UNLIMITED ON USERSDB;

-- Create the tables

CREATE TABLE rustUser.customers (
    customer_id NUMBER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    first_name NVARCHAR2(50) NOT NULL,
    last_name NVARCHAR2(50) NOT NULL,
    address NVARCHAR2(255) NOT NULL,
    email NVARCHAR2(100) NOT NULL,
    password NVARCHAR2(255) NOT NULL
);

CREATE TABLE rustUser.products (
    product_id VARCHAR2(50) PRIMARY KEY NOT NULL,
    product_name NVARCHAR2(100) NOT NULL,
    price FLOAT NOT NULL,
    category NVARCHAR2(100) NOT NULL,
    image BLOB,
    pcs_per_box NUMBER NOT NULL,
    max_boxes_per_pallet NUMBER NOT NULL,
    pcs_per_pallet NUMBER GENERATED ALWAYS AS (pcs_per_box * max_boxes_per_pallet) VIRTUAL,
    pcs_per_truck NUMBER GENERATED ALWAYS AS (pcs_per_box * max_boxes_per_pallet * 33) VIRTUAL
);

CREATE TABLE rustUser.stocks (
    product_id VARCHAR2(50) PRIMARY KEY,
    qty NUMBER NOT NULL,
    FOREIGN KEY (product_id) REFERENCES rustUser.products(product_id)
);

CREATE TABLE rustUser.orders (
    order_id NUMBER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    customer_id NUMBER,
    order_date TIMESTAMP DEFAULT SYSTIMESTAMP,
    delivery_date TIMESTAMP DEFAULT SYSTIMESTAMP + INTERVAL '1' HOUR,
    customer_address NVARCHAR2(255),
    customer_full_name NVARCHAR2(255),
    FOREIGN KEY (customer_id) REFERENCES rustUser.customers(customer_id)
);

CREATE TABLE rustUser.order_details (
    detail_id NUMBER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    order_id NUMBER,
    product_id VARCHAR2(50) NOT NULL,
    product_name NVARCHAR2(100) NOT NULL,
    quantity NUMBER NOT NULL,
    total_price FLOAT,
    total_price_tax FLOAT,
    FOREIGN KEY (order_id) REFERENCES rustUser.orders(order_id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES rustUser.products(product_id)
);
