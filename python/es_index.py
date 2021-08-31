import argparse
import json
from elasticsearch import Elasticsearch, RequestError, helpers
from elasticsearch_dsl import Search

es = Elasticsearch()


def create_index(index: str, input_file: str) -> None:
    if es.indices.exists(index):
        raise Exception("index '%s' already exists" % index)
    mappings = json.load(open(input_file))
    resp = es.indices.create(index=index, body=mappings)
    print(resp)


def upsert_records(index: str, input_file: str) -> None:
    with open(input_file) as f:
        records = [json.loads(l) for l in f]
        for r in records:
            r["_index"] = index
    resp = helpers.bulk(es, records)
    print(resp)


def delete_index(index: str) -> None:
    confirm = input("do you really want to delete '%s' index? (Y/n) " % index)
    if confirm.lower() in ("y", "yes"):
        es.indices.delete(index=index)
        print("index '%s' deleted" % index)
    else:
        print("aborted.")
    

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="re-index into elasticsearch")
    parser.add_argument("--action", dest="action", 
            help="please specify whether you are creating, upinserting or deleting the index")
    parser.add_argument("--input", dest="input",
            help="please specify the input json file")
    parser.add_argument("--index", dest="index",
            help="please specify the elasticsearch index")
    args = parser.parse_args()

    if args.action not in ("create", "upsert", "delete"):
        parser.error("--action need to be either 'create', 'upsert' or 'delete'")

    if args.input and not args.input.endswith(".json"):
        parser.error("--input file need to be a json")

    if not args.index:
        parser.error("--index is required")

    if args.action == "create":
        create_index(args.index, args.input)
    if args.action == "upsert":
        upsert_records(args.index, args.input)
    if args.action == "delete":
        delete_index(args.index)
