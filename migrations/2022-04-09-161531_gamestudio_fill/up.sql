INSERT INTO Publishers (name, price, popularity)
VALUES 
('Microsoft', 228, 6),
('Nintendo', 12000, 7),
('Tencent Games', 11000, 8),
('Sony Interactive Entertainment', 10000, 9),
('Activision Blizzard', 11500, 9),
('Electronic Arts', 12000.99, 10);



INSERT INTO Games (name, genre, release_date, prime_cost, publisher_id, cost, is_subscribable)
VALUES
('GTA The Trilogy', 'Action', '2021.11.11', 1, 6, 60, false),
('FNAF5', 'Horror', '2022.01.20', 30000, 1, 20, false),
('MARIO 1337', 'Platformer', '2041.05.1', 25000, 2, 49, true),
('ФІФА 5', 'Simulator', '2019.07.6', 450000, 6, 49, false),
('The Sims 4', 'Simulator', '2016.11.10', 75000, 6, 30, false),
('The Legend Of Zelda', 'Open World', '2020.4.19', 200000, 2, 40, false),
('Sea of thieves', 'Open World', '2018.3.20', 180000, 1, 42, false),
('World Of Warcraft', 'RPG', '2012.10.29', 100000, 5, 10, true);



INSERT INTO Investors (name, is_company)
VALUES 
('Johnny James', false),
('Microsoft', true),
('Andrey Petrenko', false),
('Augustina Richy', false),
('The Old Good Times', true),
('Somewhere ent.', true),
('Jonathan Joestar', false),
('Speedwagon Foundation', true),
('James Jameson', false),
('Kira Yoshikage', false);


INSERT INTO Staff (name, birth)
VALUES
('Joohn Johnson', '1995.12.5'),
('Alexander Fedorovskyi', '2002.07.02'),
('Andrey Petrenko', '2003.09.13'),
('Alina Aivasofksi', '1996.05.9'),
('Jenifer Johnson', '1997.10.19'),
('Jolyne Kujo', '2004.05.17');


INSERT INTO Investments (investor_id, game_id, share, invested)
VALUES 
(1, 1, 10, 30000),
(2, 1, 1, 3000),
(4, 2, 40, 12500),
(10, 1, 5, 100.25),
(6, 4, 20, 80000),
(7, 7, 25, 37850),
(8, 6, 100, 1);


INSERT INTO Users (nickname, registration_date)
VALUES 
('Andrei Petrenko', '2020.12.1'),
('Yana Kriak', '2021.11.22'),
('Shurik Fedorovskyi', '2019.01.15'),
('Pavel', '2019.06.30'),
('Anonim', '2018.07.15'),
('Kitten Miuki', '2021.02.4'),
('Johnny Anderson', '2020.10.09');


INSERT INTO Donations (user_id, game_id, amount, donation_time)
VALUES
(3, 1, 50, '2004-05-23T14:25:10'),
(1, 2, 150, '2020-07-22T21:13:01'),
(4, 3, 94, '2022-01-11T10:11:59'),
(1, 2, 299, '2021-01-23T05:12:41'),
(2, 5, 254, '2021-02-11T13:19:19');


INSERT INTO Jobs (game_id, staff_id, position, first_work_day, last_work_day, salary)
VALUES
(1, 1, 'C++ Jun Dev', '2020.12.1', '2020.12.31', 1000),
(2, 1, 'C++ Jun Dev', '2021.1.1', '2021.07.5', 1400),
(2, 2, 'JAVA Jun Dev', '2020.12.1', '2021.02.1', 1000),
(2, 2, 'JAVA Mid Dev', '2021.02.2', NULL, 1950),
(3, 3, 'C++ Mid Dev', '2020.12.1', '2021.06.12', 2010),
(4, 4, 'C++ Sen Dev', '2021.06.2', '2021.11.26', 3700),
(4, 3, 'JAVA Mid Dev', '2022.01.3', NULL, 2000),
(6, 4, 'C++ Sen Dev', '2021.11.27', NULL, 3900),
(6, 1, 'C++ Mid Dev', '2020.12.1', NULL, 2000),
(6, 4, 'C++ Sen Dev', '2021.12.1', NULL, 3000),
(2, 5, 'TeamLead', '2021.5.21', NULL, 5000),
(1, 6, 'HR', '2020.01.1', NULL, 800),
(7, 6, 'HR', '2020.01.1', NULL, 800);
