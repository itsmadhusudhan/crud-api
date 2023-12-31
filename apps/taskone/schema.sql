 CREATE table tasks (
     id UUID PRIMARY KEY,
     name VARCHAR(255) NOT NULL,
     description VARCHAR(255),
     done BOOLEAN NOT NULL,
     due_date TIMESTAMP NOT NULL
     created_at TIMESTAMP NOT NULL,
     updated_at TIMESTAMP NOT NULL,
     deleted_at TIMESTAMP,
 );
