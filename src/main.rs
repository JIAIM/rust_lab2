#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::content::RawHtml;

#[derive(FromForm)]
struct Calculator {
    a: f64,
    b: f64,
    operation: String,
}

impl Calculator {
    fn add(&self) -> f64 {
        self.a + self.b
    }

    fn subtract(&self) -> f64 {
        self.a - self.b
    }

    fn multiply(&self) -> f64 {
        self.a * self.b
    }

    fn divide(&self) -> Result<f64, String> {
        if self.b == 0.0 {
            Err("Помилка: Ділення на нуль".to_string())
        } else {
            Ok(self.a / self.b)
        }
    }
}

fn cal(calculator: &Calculator) -> Result<f64, String> {
    match calculator.operation.as_str() {
        "+" => Ok(calculator.add()),
        "-" => Ok(calculator.subtract()),
        "*" => Ok(calculator.multiply()),
        "/" => calculator.divide(),
        _ => Err("Невідома операція".to_string()),
    }
}

#[post("/result", data = "<calculator_form>")]
fn result(calculator_form: Form<Calculator>) -> RawHtml<String> {
    let calculator = calculator_form.into_inner();
    let res = match cal(&calculator) {
        Ok(res) => res,
        Err(e) => return RawHtml(format!("<p style='font-size:3em; text-align:center;'>{}</p>", e)),
    };
    RawHtml(format!("<p style='font-size:3em; text-align:center;'>Результат: {}</p>", res))
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(r#"
    <html>
    <head>
        <title>Калькулятор</title>
        <style>
            body {
                background: white;
                margin: 0;
                padding: 50px;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
            }
            .calculator {
                background: rgba(255, 255, 255, 0.9);
                border-radius: 12px;
                box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
                padding: 30px;
                width: 320px;
                text-align: center;
            }
            h1 {
                color: black;
                margin-bottom: 20px;
            }
            input {
                width: 100%;
                padding: 10px;
                margin: 10px 0;
                border: 2px solid #ccc;
                border-radius: 6px;
                font-size: 16px;
            }
            .operation-buttons {
                display: flex;
                justify-content: space-around;
                margin: 10px 0;
            }
            button {
                background-color: #007bff;
                color: white;
                border: none;
                cursor: pointer;
                padding: 10px 20px;
                border-radius: 6px;
                transition: background-color 0.3s;
            }
            button:hover {
                background-color: #0056b3;
            }
        </style>
    </head>
    <body>
        <div class="calculator">
            <h1>Калькулятор</h1>
            <form name='calculator_form' action='/result' method='post'>
                <input type='number' step='any' name='a' placeholder='a' required>
                <input type='number' step='any' name='b' placeholder='b' required>
                <div class="operation-buttons">
                    <button type='submit' name='operation' value='+'>+</button>
                    <button type='submit' name='operation' value='-'>-</button>
                    <button type='submit' name='operation' value='*'>*</button>
                    <button type='submit' name='operation' value='/'>/</button>
                </div>
            </form>
        </div>
    </body>
    </html>
    "#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, result])
}
