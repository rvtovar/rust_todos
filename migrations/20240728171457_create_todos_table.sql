CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            description TEXT NOT NULL,
            status BOOLEAN NOT NULL
        );
