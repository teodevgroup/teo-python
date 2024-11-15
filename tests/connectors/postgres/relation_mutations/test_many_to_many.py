from teo import App
from teo.test import TestCase
from teo.test.matchers import match_json_value, ignore, one_matches
from tests.helpers.schema_path_args import schema_path_args
from tests.helpers.builtin_req import builtin_req


class TestManyToMany(TestCase):

    @classmethod
    def load_app(cls):
        return App(schema_path_args(__file__, "schema.teo"))

    async def test_create_with_nested_create_one(self):
        _create_res = await builtin_req(self.server, "create", "Artist", {
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "create": {
                        "name": "Love Story"
                    }
                }
            },
        })
        find_many_res = await builtin_req(self.server, "findMany", "Artist", {
            "include": {
                "songs": True
            }
        })
        match_json_value(find_many_res, {
            "meta": {
                "count": 3
            },
            "data": one_matches({
                "id": ignore,
                "name": "Taylor Swift",
                "songs": one_matches({
                    "id": ignore,
                    "name": "Love Story"
                })
            })
        })
