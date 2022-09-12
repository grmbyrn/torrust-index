CREATE TABLE IF NOT EXISTS torrust_torrent_announce_urls (
    announce_url_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    torrent_id INTEGER NOT NULL,
    tracker_url TEXT NOT NULL,
    FOREIGN KEY(torrent_id) REFERENCES torrust_torrents(torrent_id) ON DELETE CASCADE
)
