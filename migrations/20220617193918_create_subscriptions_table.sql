CREATE TABLE subscription(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    subscribed_at timestamptz NOT NULL
);