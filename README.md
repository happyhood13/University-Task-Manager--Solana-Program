🎓 University Task Manager (Solana Program)
Este proyecto es un gestor de tareas universitarias descentralizado desarrollado en Rust utilizando el framework Anchor. Fue creado como proyecto final para la Certificación de Solana Developer.

📝 Descripción
Como estudiante, la organización es clave. Este programa permite gestionar los pendientes de la universidad directamente en la blockchain de Solana. El sistema permite organizar las tareas por materia y asignarles una fecha de vencimiento, asegurando que cada registro sea único, inmutable y propiedad exclusiva del estudiante.

Características Principales:
Organización por Materia: Cada tarea está vinculada a una asignatura específica.

Control de Tiempo: Registro de fechas de entrega mediante timestamps de Unix.

Seguridad: Solo el creador de la tarea (el estudiante) tiene permisos para modificarla o marcarla como completada.

Eficiencia de Datos: Uso de PDAs para un almacenamiento estructurado y económico.

🛠 Especificaciones Técnicas (Requisitos de Certificación)
1. CRUD Completo
El programa implementa las cuatro operaciones básicas:

Create: Función crear_tarea para inicializar el registro en la red.

Read: Consulta de estados a través de PDAs derivados.

Update: Funciones marcar_completada y reprogramar_tarea para modificar el estado de los pendientes.

Delete: Función eliminar_tarea para cerrar la cuenta y recuperar los lamports de renta.

2. Manejo de PDAs (Program Derived Addresses)
Para este proyecto, las direcciones de las tareas se derivan de forma determinista utilizando las siguientes semillas:

El string estático: "task"

La llave pública del estudiante: estudiante.key()

El ID único de la tarea: id_tarea

3. Estructura del Estado (TareaState)
Rust
pub struct TareaState {
    pub estudiante: Pubkey,      // Dueño de la tarea (32 bytes)
    pub id: u64,                // ID único (8 bytes)
    pub materia: String,        // Nombre de la materia
    pub descripcion: String,    // Detalle de la tarea
    pub fecha_vencimiento: i64, // Timestamp de entrega (8 bytes)
    pub completada: bool,       // Estado (1 byte)
}
🚀 Instalación y Uso
Requisitos
Rust

Solana CLI

Anchor Framework

Pasos
Clonar el repositorio:

Bash
git clone https://github.com/happy_hood31/university-task-manager.git
Compilar el programa:

Bash
anchor build
Ejecutar los tests:

Bash
anchor test
📄 Documentación de Código
El código fuente en programs/task-manager/src/lib.rs cuenta con comentarios detallados que explican la lógica de cada instrucción y la gestión de memoria en Solana.
