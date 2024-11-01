mod asm;
mod backend;
mod mem;

use qmetaobject::prelude::*;

qrc!(qml_resources, "qml" {
    "src/qml/main.qml" as "main.qml",
});

fn main() {
    let mut engine = QmlEngine::new();
    qml_register_type::<backend::Backend>(c"chip8.backend", 1, 0, c"Backend");
    qml_resources();
    engine.load_file(QString::from("qrc:/qml/main.qml"));
    engine.exec();
}
