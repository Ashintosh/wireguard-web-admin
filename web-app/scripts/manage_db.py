import argparse
import pathlib
import sqlite3
import sys
from argon2 import PasswordHasher

DB_EXTENSIONS = ('.sqlite', '.db')
DEFAULT_SCHEMA = """
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);
"""


def find_or_create_db(db_path=None):
    if db_path:
        path = pathlib.Path(db_path)
        if not path.exists():
            open(db_path, 'a').close()
    else:
        files = [f for f in pathlib.Path('.').iterdir() if f.suffix in DB_EXTENSIONS]
        if files:
            db_path = str(files[0])
        else:
            db_path = 'db.sqlite'
            open(db_path, 'a').close()
    conn = sqlite3.connect(db_path)
    conn.execute(DEFAULT_SCHEMA)
    conn.commit()
    return conn


def add_user(conn, ph, username, password):
    password_hash = ph.hash(password)
    try:
        conn.execute("INSERT INTO users (username, password_hash) VALUES (?, ?)", (username, password_hash))
        conn.commit()
        print("User added.")
    except sqlite3.IntegrityError:
        print("Username already exists.")


def remove_user(conn, username):
    cur = conn.execute("DELETE FROM users WHERE username=?", (username,))
    conn.commit()
    if cur.rowcount:
        print("User deleted.")
    else:
        print("User not found.")


def change_password(conn, ph, username, password):
    password_hash = ph.hash(password)
    cur = conn.execute("UPDATE users SET password_hash=? WHERE username=?", (password_hash, username))
    conn.commit()
    if cur.rowcount:
        print("Password updated.")
    else:
        print("User not found.")


def main():
    parser = argparse.ArgumentParser(description="User admin tool to manage sqlite3 database users.")
    parser.add_argument("--db", "-d", help="Path to database file (optional).")
    subparsers = parser.add_subparsers(dest="command")

    add = subparsers.add_parser("add", help="Add user")
    add.add_argument("username")
    add.add_argument("password")

    remove = subparsers.add_parser("remove", help="Remove user")
    remove.add_argument("username")

    changepw = subparsers.add_parser("changepw", help="Change user password")
    changepw.add_argument("username")
    changepw.add_argument("password")

    args = parser.parse_args()
    ph = PasswordHasher()
    conn = find_or_create_db(args.db)

    if args.command == "add":
        add_user(conn, ph, args.username, args.password)
    elif args.command == "remove":
        remove_user(conn, args.username)
    elif args.command == "changepw":
        change_password(conn, ph, args.username, args.password)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
