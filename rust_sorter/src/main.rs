use std::{
    sync::mpsc,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use rand::Rng;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Configuración de la ventana.
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    // Corre la aplicación.
    eframe::run_native(
        "Sorter",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

// Estructura de la aplicación.
struct MyApp {
    curr: i32,
    curr2: u32,
    arr: Vec<i32>,
    results: Vec<String>,
    rx: Option<mpsc::Receiver<String>>,
    empty_arr: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            curr: 0,
            curr2: 0,
            arr: vec![],
            results: vec![],
            rx: None,
            empty_arr: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SORTER");

            ui.horizontal(|ui| {
                ui.label("Add an element to the array: ");

                ui.add(egui::DragValue::new(&mut self.curr));

                let add_b = ui.button("Add");
                if add_b.clicked() {
                    self.arr.push(self.curr);
                    self.curr = 0;
                }

                ui.label(format!("Array: {:?}", self.arr));
            });

            ui.label("OR...");

            ui.horizontal(|ui| {
                ui.label("How many random elements you want in the array?");

                ui.add(egui::DragValue::new(&mut self.curr2));

                let add_ran = ui.button("Add");
                if add_ran.clicked() {
                    self.arr = rand_arr(self.curr2);
                }
            });

            let sort_b = ui.button("SORT!!!");

            ui.label("Results: ");

            // Mostrar los resultados
            for result in &self.results {
                ui.label(result);
            }

            if sort_b.clicked() {
                self.results.clear();
                if self.arr.is_empty() {
                    self.empty_arr = true;
                } else {
                    let (tx, rx) = mpsc::channel();
                    self.rx = Some(rx);

                    let aux_tx = tx.clone();
                    let aux_arr = self.arr.clone();
                    thread::spawn(move || {
                        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let sorted = bubble(aux_arr.clone());
                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let result = format!(
                            "Bubble Sort: Time Elapsed -> {:?}; Result -> {:?}",
                            (end - start),
                            sorted,
                        );
                        aux_tx.send(result).unwrap();
                    });

                    let aux_tx = tx.clone();
                    let aux_arr = self.arr.clone();
                    thread::spawn(move || {
                        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let sorted = selection(aux_arr);
                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let result = format!(
                            "Selection Sort: Time Elapsed -> {:?}; Result -> {:?}",
                            (end - start),
                            sorted,
                        );
                        aux_tx.send(result).unwrap();
                    });

                    let aux_tx = tx.clone();
                    let aux_arr = self.arr.clone();
                    thread::spawn(move || {
                        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let sorted = insertion(aux_arr);
                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let result = format!(
                            "Insertion Sort: Time Elapsed -> {:?}; Result -> {:?}",
                            (end - start),
                            sorted,
                        );
                        aux_tx.send(result).unwrap();
                    });

                    let aux_tx = tx.clone();
                    let aux_arr = self.arr.clone();
                    thread::spawn(move || {
                        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let sorted = merge(aux_arr);
                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let result = format!(
                            "Merge Sort: Time Elapsed -> {:?}; Result -> {:?}",
                            (end - start),
                            sorted,
                        );
                        aux_tx.send(result).unwrap();
                    });

                    let aux_tx = tx.clone();
                    let aux_arr = self.arr.clone();
                    thread::spawn(move || {
                        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let sorted = quick(aux_arr);
                        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let result = format!(
                            "Quick Sort: Time Elapsed -> {:?}; Result -> {:?}",
                            (end - start),
                            sorted,
                        );
                        aux_tx.send(result).unwrap();
                    });
                }
            }

            if let Some(rx) = &self.rx {
                while let Ok(result) = rx.try_recv() {
                    self.results.push(result); // Guardar permanentemente los resultados
                }
            }

            Some(())
        });

        ctx.request_repaint();
    }
}

fn bubble(arr: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = arr;

    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                (arr[j], arr[j + 1]) = (arr[j + 1], arr[j]);
            }
        }
    }
    arr
}

fn selection(arr: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = arr;

    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        (arr[i], arr[min_idx]) = (arr[min_idx], arr[i]);
    }
    arr
}

fn insertion(arr: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = arr;

    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i - 1;
        let mut neg: bool = false;
        while key < arr[j] {
            arr[j + 1] = arr[j];
            if j == 0 {
                neg = true;
                break;
            }
            j -= 1;
        }
        arr[ if neg {j} else {j + 1}] = key;
    }
    arr
}

fn merge(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;

    let n = arr.len();
    if n > 1 {
        let mid = n / 2;

        let mut l: Vec<i32> = vec![];
        arr[0..mid].clone_into(&mut l);
        let mut r: Vec<i32> = vec![];
        arr[mid..n].clone_into(&mut r);

        l = merge(l);
        r = merge(r);

        let [mut i, mut j, mut k] = [0; 3];
        while i < l.len() && j < r.len() {
            if l[i] < r[j] {
                arr[k] = l[i];
                i += 1;
            } else {
                arr[k] = r[j];
                j += 1;
            }
            k += 1;
        }

        while i < l.len() {
            arr[k] = l[i];
            i += 1;
            k += 1;
        }

        while j < r.len() {
            arr[k] = r[j];
            j += 1;
            k += 1;
        }
    }
    arr
}

fn quick(arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    if n <= 1 {
        return arr;
    } else {
        let pivot = arr[n / 2];
        let left: Vec<i32> = arr.iter().filter(|&&x| x < pivot).cloned().collect();
        let middle: Vec<i32> = arr.iter().filter(|&&x| x == pivot).cloned().collect();
        let right: Vec<i32> = arr.iter().filter(|&&x| x > pivot).cloned().collect();

        return [quick(left), middle, quick(right)].concat();
    }
}

fn rand_arr(nums: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = vec![];
    for _i in 0..nums {
        arr.push(rng.gen_range(-32768..32768));
    }
    arr
}
