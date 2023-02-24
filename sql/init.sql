CREATE DATABASE decensha
    WITH
    OWNER = admin
    ENCODING = 'UTF8'
    LC_COLLATE = 'en_US.utf8'
    LC_CTYPE = 'en_US.utf8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;



CREATE SCHEMA IF NOT EXISTS public
    AUTHORIZATION pg_database_owner;

COMMENT ON SCHEMA public
    IS 'standard public schema';

GRANT USAGE ON SCHEMA public TO PUBLIC;

GRANT ALL ON SCHEMA public TO pg_database_owner;



CREATE TABLE IF NOT EXISTS public.members
(
    id integer NOT NULL DEFAULT nextval('members_id_seq'::regclass),
    name text COLLATE pg_catalog."default" NOT NULL,
    permlevel integer NOT NULL,
    profilepic bytea,
    bio text COLLATE pg_catalog."default",
    clientid text COLLATE pg_catalog."default" NOT NULL,
    joined timestamp with time zone NOT NULL,
    accountkey text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT members_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.members
    OWNER to admin;



CREATE TABLE IF NOT EXISTS public.channels
(
    id integer NOT NULL DEFAULT nextval('channels_id_seq'::regclass),
    name text COLLATE pg_catalog."default" NOT NULL,
    index integer NOT NULL,
    CONSTRAINT channels_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.channels
    OWNER to admin;



CREATE TABLE IF NOT EXISTS public.messages
(
    id integer NOT NULL DEFAULT nextval('messages_id_seq'::regclass),
    message text COLLATE pg_catalog."default" NOT NULL,
    attachment bytea[],
    author integer NOT NULL,
    date timestamp with time zone NOT NULL,
    CONSTRAINT messages_pkey PRIMARY KEY (id),
    CONSTRAINT members_id FOREIGN KEY (author)
        REFERENCES public.members (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.messages
    OWNER to admin;



CREATE TABLE IF NOT EXISTS public.audit
(
    id integer NOT NULL DEFAULT nextval('audit_id_seq'::regclass),
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