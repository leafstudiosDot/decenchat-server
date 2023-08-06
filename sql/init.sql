CREATE SCHEMA admin
    AUTHORIZATION pg_database_owner;

CREATE TABLE admin.accounts
(
    id bigserial NOT NULL,
    name text NOT NULL,
    password text NOT NULL,
    permissions text[],
    status text,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS admin.accounts
    OWNER to admin;

INSERT INTO admin.accounts (name, password, permissions, status) VALUES ('Administrator', '$2y$10$XBs4NN00pxd5e01Nb01Qk.xTJQM1N0Epk4dWA/EWv7XP9Oe8e.uX6', '{"ADMINISTRATOR"}', 'master');
                                                                                        -- admin



CREATE SCHEMA IF NOT EXISTS public
    AUTHORIZATION pg_database_owner;

COMMENT ON SCHEMA public
    IS 'standard public schema';

GRANT USAGE ON SCHEMA public TO PUBLIC;

GRANT ALL ON SCHEMA public TO pg_database_owner;


CREATE SEQUENCE IF NOT EXISTS members_id_seq;

CREATE TABLE public.roles
(
    id bigserial NOT NULL,
    name text NOT NULL,
    permissions text[] NOT NULL,
    color text NOT NULL,
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.roles
    OWNER to admin;

CREATE TABLE IF NOT EXISTS public.members
(
    id bigserial NOT NULL,
    username text NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    roles bigint[],
    profilepic text,
    secretkey text NOT NULL,
    bio text COLLATE pg_catalog."default",
    clientid text COLLATE pg_catalog."default" NOT NULL,
    joined timestamp with time zone NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    recoverymethod json[],
    status text,
    CONSTRAINT members_pkey PRIMARY KEY (id),
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.members
    OWNER to admin;


CREATE SEQUENCE IF NOT EXISTS channels_id_seq;


CREATE TABLE IF NOT EXISTS public.channels
(
    id bigserial NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    sortindex integer NOT NULL,
    voice boolean,
    specificpermissions json[],
    CONSTRAINT channels_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.channels
    OWNER to admin;

INSERT INTO public.channels(
	id, name, sortindex, voice)
	VALUES (0, 'general', 0, false);

INSERT INTO public.channels(
	id, name, sortindex, voice)
	VALUES (1, 'Voice', 1, true);


CREATE SEQUENCE IF NOT EXISTS messages_id_seq;


CREATE TABLE IF NOT EXISTS public.messages
(
    id bigserial NOT NULL,
    message text COLLATE pg_catalog."default" NOT NULL,
    attachment json[],
    author bigserial NOT NULL,
    date timestamp with time zone NOT NULL,
    channel bigserial NOT NULL,
    replyto bigint,
    spoiler boolean;
    CONSTRAINT messages_pkey PRIMARY KEY (id),
    CONSTRAINT members_id FOREIGN KEY (author)
        REFERENCES public.members (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE,
    CONSTRAINT messages_channel_fkey FOREIGN KEY (channel)
        REFERENCES public.channels (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.messages
    OWNER to admin;


CREATE SEQUENCE IF NOT EXISTS audit_id_seq;


CREATE TABLE IF NOT EXISTS public.audit
(
    id bigserial NOT NULL,
    member integer NOT NULL,
    action text COLLATE pg_catalog."default" NOT NULL,
    tag text COLLATE pg_catalog."default" NOT NULL,
    date timestamp with time zone,
    CONSTRAINT audit_pkey PRIMARY KEY (id),
    CONSTRAINT member FOREIGN KEY (member)
        REFERENCES public.members (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.audit
    OWNER to admin;