# Flickr Indexer for `copostr`

This is a script for downloading indices of Flickr images for use with `copostr`.

## Setup

This script needs the following:

- Python 3
- the Python library `flickr_api`
    - this can be installed with `pip install -r requirements.txt`
- Flickr API credentials (you can get them [here](https://www.flickr.com/services/apps/create/))

Copy `config.example.json` to `config.json` and edit it.

```json
{
    // use the credentials given by Flickr
    "api_key": "API_KEY_HERE",
    "api_secret": "API_SECRET_HERE",
    // a path to a directory where the script can write out Flickr indices
    "out_path": "out",
    // number of results per index
    "result_limit": 1000,
    // each query has a name that will be used for the index file 
    "queries": {
        "query1": [{
            // here are the query arguments for the Flickr API request
            // keys and values should match the arguments listed here:
            // https://www.flickr.com/services/api/flickr.photos.search.html
            // by default, the script will include the license, content_type,
            // per_page, and extras arguments to only return images that are
            // Creative Commons, Public Domain, or Flickr Commons as well as
            // some other necessary things. Don't include those arguments here.
            // `group_name` is a special convenience argument that will be
            // converted to a `group_id` internally.
            //
            // The example below will do a keyword search for "something" in
            // the group named "FOO_BAR_CHANGE_ME".
            "group_name": "FOO_BAR_CHANGE_ME"
        }],
        // Multiple queries can be grouped so that they go to the same database.
        "query2": [
            {
                "group_id": "123@N12",
                "text": "foo"
            },
            {
                "text": "bar"
            }
        ]
    }
}

```

## Usage

```
python3 flickr-indexer.py
```

## Output

The `out_path` directory will contain directories with sqlite databases and template credentials files for running copostr.
There will be one directory per query.

For example:

```
out_path
├── query1
│   ├── credentials
│   └── images.db
└── query2
    ├── credentials
    └── images.db
```

This hierarchy is designed to be read-to-use with `copostr`.

### Schema

The schema for each sqlite database is:

| Column    | Type   | Description | Example value |
|:----------|:-------|:------------|:--------------|
| `id`      | `INT`  | Image ID (primary key) | `52488299990` |
| `title`   | `TEXT` | Image title | `Framed snow leopard` |
| `source`  | `TEXT` | Source URL for image | `https://www.flickr.com/photos/8070463@N03/52488299990/` |
| `image`   | `TEXT` | Direct URL to the image file | `https://live.staticflickr.com/65535/52488299990_ab31e9a282_b.jpg` |
| `license` | `TEXT` | Image licence | `Attribution-NoDerivs License` |
| `status`  | `INT`  | Posted ltatus | `1` |

`status` is an enum defined in `copostr`.
