from teo import App, Request, Response, HandlerGroup
from tests.helpers.schema_path_args import schema_path_args
from .entities import SupportCreateInput, SupportFindManyArgs, Teo


def load_app():
    app = App(schema_path_args(__file__, "schema.teo"))
    def support_handler_group(group: HandlerGroup):
        async def my_create_object(req: Request):
            teo: Teo = req.teo()
            input: SupportCreateInput = req.body_object()
            object = await teo.support.create_object(input)
            await object.save()
            return Response.data(await object.to_teon())
        group.define_handler("myCreateObject", my_create_object)
        async def my_find_many_objects(req: Request):
            teo: Teo = req.teo()
            input: SupportFindManyArgs = req.body_object()
            objects = await teo.support.find_many_objects(input)
            values = [await object.to_teon() for object in objects]
            return Response.data(values)
        group.define_handler("myFindManyObjects", my_find_many_objects)
    app.main_namespace().define_model_handler_group("Support", support_handler_group)
    return app
