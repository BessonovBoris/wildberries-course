CREATE TABLE IF NOT EXISTS orders
(
    order_uid            VARCHAR NOT NULL PRIMARY KEY,
    track_number         VARCHAR,
    entry                VARCHAR,
    locale               VARCHAR,
    internal_signature   VARCHAR,
    customer_id          VARCHAR,
    delivery_service     VARCHAR,
    shardkey             VARCHAR,
    sm_id                BIGINT,
    date_created         TIMESTAMP WITH TIME ZONE,
    oof_shard            VARCHAR
);

CREATE TABLE IF NOT EXISTS delivery
(
    order_uid   VARCHAR NOT NULL PRIMARY KEY,
    name        VARCHAR,
    phone       VARCHAR,
    zip         VARCHAR,
    city        VARCHAR,
    address     VARCHAR,
    region      VARCHAR,
    email       VARCHAR,
    FOREIGN KEY (order_uid) REFERENCES orders (order_uid)
    ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS products
(
    order_uid       VARCHAR NOT NULL,
    chrt_id         BIGINT,
    track_number    VARCHAR,
    price           BIGINT,
    rid             VARCHAR,
    name            VARCHAR,
    sale            BIGINT,
    size            VARCHAR,
    total_price     BIGINT,
    nm_id           BIGINT,
    brand           VARCHAR,
    status          BIGINT,
    FOREIGN KEY (order_uid) REFERENCES orders (order_uid)
    ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS payments
(
    transaction     VARCHAR NOT NULL PRIMARY KEY,
    request_id      VARCHAR,
    currency        VARCHAR,
    provider        VARCHAR,
    amount          BIGINT,
    payment_dt      BIGINT,
    bank            VARCHAR,
    delivery_cost   BIGINT,
    goods_total     BIGINT,
    custom_fee      BIGINT,
    FOREIGN KEY (transaction) REFERENCES orders (order_uid)
    ON DELETE CASCADE
    );

INSERT INTO orders (order_uid, track_number, entry, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard)
VALUES ('b563feb7b2b84b6test', 'WBILMTESTTRACK', 'WBIL', 'en', '', 'test', 'meest', '9', 99, '2021-11-26T06:22:19Z', '1');

INSERT INTO delivery (order_uid, name, phone, zip, city, address, region, email)
VALUES ('b563feb7b2b84b6test', 'Test Testov', '+9720000000', '2639809', 'Kiryat Mozkin', 'Ploshad Mira 15', 'Kraiot', 'test@gmail.com');

INSERT INTO products (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
VALUES
    ('b563feb7b2b84b6test', 9934930, 'WBILMTESTTRACK', 453, 'ab4219087a764ae0btest', 'Mascaras', 30, '0', 317, 2389212, 'Vivienne Sabo', 202),
    ('b563feb7b2b84b6test', 9934931, 'WBILMTESTTRACK', 228, 'ab4219087a7test0btest', 'Smartphone Vivo', 30, '0', 317, 2389212, 'Vivo', 202);

INSERT INTO payments (transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
VALUES ('b563feb7b2b84b6test', '', 'USD', 'wbpay', 1817, 1637907727, 'alpha', 1500, 317, 0);