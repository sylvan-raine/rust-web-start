CREATE TABLE department(
    id                              CHAR(2)        NOT NULL UNIQUE,
    name                            VARCHAR(20),
    office_room                     VARCHAR(40),
    home_page                       VARCHAR(80),
    PRIMARY KEY (id)
);

CREATE TABLE student(
    id 		                        CHAR(6)         NOT NULL UNIQUE,
    name		                    VARCHAR(20)     NOT NULL,
    sex		                        CHAR(2)         CHECK (sex IN('男','女')),
    age		                        INT,
    email                           VARCHAR(50),
    department_id		            CHAR(2),
    PRIMARY KEY (id),
    FOREIGN KEY (department_id)     REFERENCES      department(id)
);

CREATE TABLE course(
    id		                        CHAR(6)	        NOT NULL UNIQUE,
    name		                    VARCHAR(20)     NOT NULL,
    pre_course		                CHAR(6),
    credit	INT,
    department_id                   CHAR(2),
    PRIMARY KEY (id),
    FOREIGN KEY (pre_course)        REFERENCES      course(id),
    FOREIGN KEY (department_id)     REFERENCES      department(id)
);

CREATE TABLE score(
    stu_id		                    CHAR(6)	        NOT NULL,
    course_id		                CHAR(6)	        NOT NULL,
    score		                    INT,
    record_date                     date            DEFAULT current_date,
    PRIMARY KEY (stu_id, course_id),
    FOREIGN KEY (stu_id)            REFERENCES      student(id),
    FOREIGN KEY (course_id)         REFERENCES      course(id)
);

CREATE TABLE users(
    id          VARCHAR(32),
    name        VARCHAR(32) NOT NULL,
    password    VARCHAR(128) NOT NULL,
    PRIMARY KEY (id)
);

CREATE VIEW student_score_course(stu_name, stu_id, score, course_name, course_id, record_date) AS
SELECT s.name, s.id, sc.score, c.name, c.id, sc.record_date
FROM student s, score sc, course c
WHERE s.id = sc.stu_id AND c.id = sc.course_id;

-- DROP VIEW student_score_course;
-- DROP TABLE score;
-- DROP TABLE student;
-- DROP TABLE course;
-- DROP TABLE department;
-- DROP TABLE users;