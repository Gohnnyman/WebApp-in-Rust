USE GameStudio
GO

INSERT INTO Publishers
VALUES 
('Microsoft', 228, 6),
('Nintendo', 12000, 7),
('Tencent Games', 11000, 8),
('Sony Interactive Entertainment', 10000, 9),
('Activision Blizzard', 11500, 9),
('Electronic Arts', 12000.99, 10)
GO


INSERT INTO Games
VALUES
('GTA The Trilogy', 'Action', '2021.11.11', 1, 6, 60, 0),
('FNAF5', 'Horror', '2022.01.20', 30000, 1, 20, 0),
('MARIO 1337', 'Platformer', '2041.05.1', 25000, 2, 49, 0),
('FIFA 5', 'Simulator', '2019.07.6', 450000, 6, 49, 0),
('The Sims 4', 'Simulator', '2016.11.10', 75000, 6, 30, 0),
('The Legend Of Zelda', 'Open World', '2020.4.19', 200000, 2, 40, 0),
('Sea of thieves', 'Open World', '2018.3.20', 180000, 1, 42, 0),
('World Of Warcraft', 'RPG', '2012.10.29', 100000, 5, 10, 1)
GO

INSERT INTO Investors 
VALUES 
('Johnny James', 0),
('Microsoft', 1),
('Andrey Petrenko', 0),
('Augustina Richy', 0),
('The Old Good Times', 1),
('Somewhere ent.', 1),
('Jonathan Joestar', 0),
('Speedwagon Foundation', 1),
('James Jameson', 0),
('Kira Yoshikage', 0)
GO

INSERT INTO Staff 
VALUES
('Joohn Johnson', '1995.12.5'),
('Alexander Fedorovskyi', '2002.07.02'),
('Andrey Petrenko', '2003.09.13'),
('Alina Aivasofksi', '1996.05.9'),
('Jenifer Johnson', '1997.10.19'),
('Jolyne Kujo', '2004.05.17')
GO

INSERT INTO Investor_Game 
VALUES 
(1, 1, 10, 30000),
(2, 1, 1, 3000),
(4, 2, 40, 12500),
(10, 1, 5, 100.25),
(6, 4, 20, 80000),
(7, 7, 25, 37850),
(8, 6, 100, 1)
GO

INSERT INTO Donations 
VALUES 
('Andrei Petrenko', 1, 28),
('Yana Kriak', 4, 123),
('Shurik Fedorovskyi', 1, 0.59),
('Pavel', 6, 338),
('Anonim', 8, 2),
('Kitten Miuki', 5, 125),
('Johnny Anderson', 4, 30)
GO

INSERT INTO Jobs
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
(7, 6, 'HR', '2020.01.1', NULL, 800)
GO


SELECT * FROM Publishers
SELECT * FROM Games
SELECT * FROM Investors
SELECT * FROM Staff
SELECT * FROM Investor_Game
SELECT * FROM Donations
SELECT * FROM Jobs
GO