#!/usr/bin/env python3

import json
import sqlite3
from pathlib import Path

import flickr_api  # type: ignore

CRED_FILE = "COPOSTR_EMAIL=''\nCOPOSTR_PASSWORD=''\nCOPOSTR_PROJECT=''\n"


def run_flickr_query(args: dict):
    common_query_args = {
        # only get CC/Public Domain images
        "license": "1,2,3,4,5,6,7,9,10",
        # only get images
        "content_type": "1",
        # get as many images as possible
        "per_page": "500",
        # include licence in results
        "extras": ["license"],
    }

    if "group_name" in args:
        args["group_id"] = flickr_api.Group.getByUrl(f"https://flickr.com/groups/{args['group_name']}").get("id")
        del args["group_name"]

    res = flickr_api.Walker(flickr_api.Photo.search, **(args | common_query_args))
    return res


def create_table(cur: sqlite3.Cursor):
    cur.execute("CREATE TABLE IF NOT EXISTS images(id INT PRIMARY KEY, title, source, image, license, status INT)")


def results_to_db(con: sqlite3.Connection, res: flickr_api.objects.Walker, limit: int):
    records: list[tuple] = []
    ids: set[int] = set()
    idx = 0

    for photo in res:
        if idx >= limit:
            break

        owner_id = photo.get("owner").get("id")
        photo_id = int(photo.get("id"))
        if photo_id not in ids and (image_url := photo.get("url_l")) is not None:
            ids.add(photo_id)
            records.append((
                photo_id,  # id
                photo.get("title"),  # title
                f"https://www.flickr.com/photos/{owner_id}/{photo_id}/",  # source
                image_url,  # image
                licenses.get(photo.get("license"), "Unknown License"),  # license
                0,  # status
            ))
            idx += 1

    cur = con.cursor()
    cur.executemany("INSERT OR IGNORE INTO images VALUES(?, ?, ?, ?, ?, ?)", records)
    con.commit()


if __name__ == "__main__":
    conf_file = Path("config.json")
    config = json.load(conf_file.open())

    out_path = Path(config["out_path"])
    out_path.mkdir(parents=True, exist_ok=True)

    flickr_api.set_keys(api_key=config["api_key"], api_secret=config["api_secret"])
    licenses = {str(x.get("id")): x.get("name") for x in flickr_api.License.getList()}

    for name, queries in config["queries"].items():
        out_dir = out_path / Path(f"{name}/")
        out_dir.mkdir(parents=True, exist_ok=True)

        con = sqlite3.connect(out_dir / Path("images.db"))
        create_table(con.cursor())
        for query in queries:
            results_to_db(con, run_flickr_query(query), config["result_limit"])
        con.close()

        if not (creds := out_dir / Path("credentials")).exists():
            creds.write_text(CRED_FILE)
