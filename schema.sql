--
-- PostgreSQL database dump
--

-- Dumped from database version 14.8
-- Dumped by pg_dump version 14.8

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: authors; Type: TABLE; Schema: public; Owner: guidomb
--

CREATE TABLE public.authors (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    email text NOT NULL,
    first_name text NOT NULL,
    last_name text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    username character(50) NOT NULL
);


ALTER TABLE public.authors OWNER TO guidomb;

--
-- Name: posts; Type: TABLE; Schema: public; Owner: guidomb
--

CREATE TABLE public.posts (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    title character(100) NOT NULL,
    description text NOT NULL,
    content text NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    author_id uuid NOT NULL
);


ALTER TABLE public.posts OWNER TO guidomb;

--
-- Name: authors authors_email_key; Type: CONSTRAINT; Schema: public; Owner: guidomb
--

ALTER TABLE ONLY public.authors
    ADD CONSTRAINT authors_email_key UNIQUE (email);


--
-- Name: authors authors_username_key; Type: CONSTRAINT; Schema: public; Owner: guidomb
--

ALTER TABLE ONLY public.authors
    ADD CONSTRAINT authors_username_key UNIQUE (username);


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: guidomb
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id);


--
-- Name: authors untitled_table_pkey; Type: CONSTRAINT; Schema: public; Owner: guidomb
--

ALTER TABLE ONLY public.authors
    ADD CONSTRAINT untitled_table_pkey PRIMARY KEY (id);


--
-- Name: posts post_author; Type: FK CONSTRAINT; Schema: public; Owner: guidomb
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT post_author FOREIGN KEY (author_id) REFERENCES public.authors(id);


--
-- PostgreSQL database dump complete
--

