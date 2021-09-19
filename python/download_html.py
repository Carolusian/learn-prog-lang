# -*- coding: utf-8 -*-
__author__ = "Carolusian, https://github.com/carolusian"
__license__ = "GPL"
__version__ = "1.0.0"

import argparse
import os
import re
import requests
from bs4 import BeautifulSoup
from pathlib import Path
from urllib.parse import urlparse
from urllib.request import urlretrieve


def http_content(url: str) -> str:
    return requests.get(url).text


def file_content(file_path: str) -> str:
    with open(file_path) as f:
        return f.read()


def get_content(url_or_file: str) -> str:
    http_reg = re.compile("https?://")
    if re.search(http_reg, url_or_file):
        return http_content(url_or_file)
    else:
        return file_content(url_or_file)


def download_image(image_url: str, out_folder: str) -> str:
    p = urlparse(image_url)
    file_name = os.path.basename(p.path)
    image_path = f"{out_folder}/{file_name}"
    urlretrieve(image_url, image_path)
    return file_name


def main(url_or_file: str, out_folder: str) -> None:
    soup = BeautifulSoup(get_content(url_or_file), "html.parser")

    # create the root folder
    Path(out_folder).mkdir(parents=True, exist_ok=True)

    # get images
    images = soup.findAll("img")
    if images:
        for img in images:
            image_path = download_image(img["src"], out_folder)
            img["src"] = image_path

    with open(out_folder + "/index.html", "w") as f:
        f.write(soup.prettify())


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--input")
    parser.add_argument("--output")
    args = parser.parse_args()
    main(args.input, args.output)
