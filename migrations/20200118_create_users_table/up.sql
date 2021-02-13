CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    surname VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    pw VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);

INSERT INTO users (id, name, surname, email, pw, role)
VALUES (0, "Alex", "R", "test@web.de", "admin", "admin");

INSERT INTO users (id, name, surname, email, pw, role)
VALUES (1, "Hans", "Heinrich", "test@web.de", "user", "user");
