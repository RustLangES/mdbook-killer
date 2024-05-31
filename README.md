https://github.com/Phosphorus-M/mdbook-killer/assets/19656993/fa0e8e93-7529-4e0e-9749-539f25b87a55
> This is a early version of this project

## Mdbook Killer

Este es un proyecto de experimentación 🔬🧪

USE BAJO SU PROPIO RIESGO!!1!!

## FLUJO MOMENTANEO

De momento no esta completo ningún flujo del proyecto pero para ir commiteando 
cosas el flujo es hacer cambios en template representa que tendras que ejecutar:

```
npx tailwindcss -i ./input.css -o ./leptos_start.css
```

Y luego compilar con:

```
cargo install mdbook-killer --path .
```

Entonces puedes crear una carpeta  
```
mkdir Prueba && cd Prueba
```

Y ejecutar el comando:

```
mdbook-killer init .
```

Eso creara una estructura básica. 

Por ahora solo funciona el flujo de build y de init.
Buildear utiliza el template del mismo proyecto por ahora.

```
mdbook-killer build . 
```

Como aún no hay hotreloading ni serve ni nada utilicen lo que quieran para 
levantar el compilado.
En mi caso estoy usando 

```
python3 -m http.server -d out/book
```

## Requirements
- libssl
