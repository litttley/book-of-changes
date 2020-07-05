-- Your SQL goes here
CREATE TABLE `user` (
  `userid` int(11) NOT NULL AUTO_INCREMENT,

  `username` varchar(60) NOT NULL DEFAULT '',

  PRIMARY KEY (`userid`),

  KEY `username` (`username`)

) ENGINE=MyISAM AUTO_INCREMENT=4 DEFAULT CHARSET=utf8;