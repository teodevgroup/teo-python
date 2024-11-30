from teo.test import TestCase, TestRequest
from teo.test.matchers import match_json_value, ignore, date_value, datetime_value, decimal_value, partial
from .app import load_app


class TestTypes(TestCase):

    @classmethod
    def load_app(cls):
        return load_app()
    
    async def test_transform_int32(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int32': 1
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int32": 10
            }),
        })

    async def test_transform_int64(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int64': 1
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int64": 10
            }),
        })

    async def test_transform_float32(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float32': 1.0
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float32": 10.0
            }),
        })

    async def test_transform_float64(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float64': 1.0
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float64": 10.0
            }),
        })

    async def test_transform_bool(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'bool': False
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "bool": True
            }),
        })

    async def test_transform_string(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'string': 'Love'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "string": '*Love*'
            }),
        })

    async def test_transform_date(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'date': '2005-06-01'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "date": date_value('2005-06-02')
            }),
        })

    async def test_transform_date_time(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateTime': '2024-11-29T14:49:13.498Z'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "dateTime": datetime_value("2024-11-30T14:49:13.498Z")
            }),
        })

    async def test_transform_decimal(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'decimal': '1'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "decimal": decimal_value("10")
            }),
        })

    async def test_transform_status(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'status': 'open'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "status": 'pending'
            }),
        })

    async def test_transform_int32_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int32Array': [1, 1]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int32Array": [10, 10]
            }),
        })

    async def test_transform_int64_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int64Array': [1, 1]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int64Array": [10, 10]
            }),
        })

    async def test_transform_float32_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float32Array': [1.0, 1.0]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float32Array": [10.0, 10.0]
            }),
        })

    async def test_transform_float64_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float64Array': [1.0, 1.0]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float64Array": [10.0, 10.0]
            }),
        })

    async def test_transform_bool_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'boolArray': [False, False]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "boolArray": [True, True]
            }),
        })

    async def test_transform_string_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'stringArray': ['Love', 'Love']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "stringArray": ['*Love*', '*Love*']
            }),
        })

    async def test_transform_date_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateArray': ['2005-06-01', '2005-06-01']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "dateArray": [date_value('2005-06-02'), date_value('2005-06-02')]
            }),
        })

    async def test_transform_date_time_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateTimeArray': ['2024-11-29T14:49:13.498Z', '2024-11-29T14:49:13.498Z']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "dateTimeArray": [datetime_value("2024-11-30T14:49:13.498Z"), datetime_value("2024-11-30T14:49:13.498Z")]
            }),
        })

    async def test_transform_decimal_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'decimalArray': ['1', '1']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "decimalArray": [decimal_value('10'), decimal_value('10')]
            }),
        })

    async def test_transform_status_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'statusArray': ['open', 'open']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "statusArray": ['pending', 'pending']
            }),
        })

    async def test_alter_int32(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int32': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int32": 5
            }),
        })

    async def test_alter_int64(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int64': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int64": 5
            }),
        })

    async def test_alter_float32(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float32': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float32": 5.5
            }),
        })

    async def test_alter_float64(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float64': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float64": 5.5
            }),
        })

    async def test_alter_bool(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'bool': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "bool": True
            }),
        })

    async def test_alter_string(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'string': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "string": 'Flower'
            }),
        })

    async def test_alter_date(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'date': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "date": date_value("2003-06-23")
            }),
        })

    async def test_alter_date_time(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateTime': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "dateTime": datetime_value("2004-07-23T11:30:00.000Z")
            }),
        })

    async def test_alter_decimal(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'decimal': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "decimal": decimal_value("5")
            }),
        })

    async def test_alter_status(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'status': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "status": 'done'
            }),
        })

    async def test_alter_int32_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int32Array': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int32Array": [5, 5, 5, 5, 5]
            }),
        })

    async def test_alter_int64_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int64Array': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "int64Array": [5, 5, 5, 5, 5]
            }),
        })

    async def test_alter_float32_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float32Array': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float32Array": [5.5, 5.5, 5.5, 5.5, 5.5]
            }),
        })

    async def test_alter_float64_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float64Array': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "float64Array": [5.5, 5.5, 5.5, 5.5, 5.5]
            }),
        })

    async def test_alter_bool_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'boolArray': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "boolArray": [True, False, True, False, True]
            }),
        })

    async def test_alter_string_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'stringArray': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "stringArray": ["Sing", "Dance", "Gift"]
            }),
        })

    async def test_alter_date_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateArray': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "dateArray": [date_value("2003-06-23"), date_value("2003-06-23")]
            }),
        })

    async def test_alter_date_time_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateTimeArray': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "dateTimeArray": [datetime_value("2004-07-23T11:30:00.000Z"), datetime_value("2004-07-23T11:30:00.000Z")]
            }),
        })

    async def test_alter_decimal_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'decimalArray': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "decimalArray": [decimal_value("5"), decimal_value("5"), decimal_value("5"), decimal_value("5"), decimal_value("5")]
            }),
        })

    async def test_alter_status_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'statusArray': None
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "statusArray": ["open", "inProgress", "pending", "waitingForReview", "done"]
            }),
        })

    async def test_callback_int32(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int32': 1
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "1"
            }),
        })

    async def test_callback_int64(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int64': 1
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "1"
            }),
        })

    async def test_callback_float32(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float32': 1.0
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "1.0"
            }),
        })

    async def test_callback_float64(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float64': 1.0
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "1.0"
            }),
        })

    async def test_callback_bool(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'bool': False
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "False"
            }),
        })

    async def test_callback_string(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'string': 'Love'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "Love"
            }),
        })

    async def test_callback_date(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'date': '2005-06-01'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "2005-06-01"
            }),
        })

    async def test_callback_date_time(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateTime': '2024-11-29T14:49:13.498Z'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "2024-11-29 14:49:13.498000+00:00"
            }),
        })

    async def test_callback_decimal(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'decimal': '1'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "1"
            }),
        })

    async def test_callback_status(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'status': 'open'
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "open"
            }),
        })

    async def test_callback_int32_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int32Array': [1, 1]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[1, 1]"
            }),
        })

    async def test_callback_int64_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'int64Array': [1, 1]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[1, 1]"
            }),
        })

    async def test_callback_float32_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float32Array': [1.0, 1.0]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[1.0, 1.0]"
            }),
        })

    async def test_callback_float64_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'float64Array': [1.0, 1.0]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[1.0, 1.0]"
            }),
        })

    async def test_callback_bool_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'boolArray': [False, False]
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[False, False]"
            }),
        })

    async def test_callback_string_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'stringArray': ['Love', 'Love']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "['Love', 'Love']"
            }),
        })

    async def test_callback_date_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateArray': ['2005-06-01', '2005-06-01']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[datetime.date(2005, 6, 1), datetime.date(2005, 6, 1)]"
            }),
        })

    async def test_callback_date_time_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'dateTimeArray': ['2024-11-29T14:49:13.498Z', '2024-11-29T14:49:13.498Z']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[datetime.datetime(2024, 11, 29, 14, 49, 13, 498000, tzinfo=datetime.timezone.utc), datetime.datetime(2024, 11, 29, 14, 49, 13, 498000, tzinfo=datetime.timezone.utc)]"
            }),
        })

    async def test_callback_decimal_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'decimalArray': ['1', '1']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "[Decimal('1'), Decimal('1')]"
            }),
        })

    async def test_callback_status_array(self):
        request = TestRequest(
            method='POST', 
            uri='/Container/create',
            body={
                'create': {
                    'statusArray': ['open', 'open']
                }
            })
        response = await self.server.process(request)
        match_json_value(response.body_as_json(), {
            "data": partial({
                "id": ignore,
                "message": "['open', 'open']"
            }),
        })