CREATE TABLE users (
    uuid serial NOT NULL,
    name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    maried boolean,
    removed boolean NOT NULL,
    gender text NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (uuid),
    CONSTRAINT users_email_unique UNIQUE (email)
);
