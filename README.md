# Challengue Fintelligentia

## Componentes

- MarketSimulator
- MarketDataManager
- MarketMaker

## Modelo

### MarketSimulator

Para este componente decidi hacer que las cantidades sean aleatorias, tanto para el bid price como para el ask price.
Sin embargo, cada uno de los TOB generados tiene relacion con los precios generados en el TOB anterior. Para hacerlo un poco mas "real".

Con respecto a el modelo en el codigo, he creado un trait DataProvider que se debe implementar para agregar nuevos proovedores de datos.
Esto hace que el codigo sea escalable, en caso de querer agregar nuevos proovedores.

Una cosa a tener en cuenta es que esta hardcodeado cada cuanto tiempo generara datos, actualmente seteado en 500 ms.

### MarketDataManager

Este componente tiene una lista de consumidores y una lista de proovedores de datos. Al iniciar crea un canal el cual
se usara para recibir los TOB de manera concurrente desde los distintos proovedores de datos.
Luego dispara cada uno de los proovedores en un hilo distinto para poder recibir todos los datos de manera concurrente. A cada
uno de los proovedores se les pasa el sender del canal creado previamente para poder recibir los datos de manera concurrente.

### MarketMaker

El `MarketMaker`, en este caso denominado `FuturesContractConsumer`, es un componente encargado de recibir los datos 
de mercado en forma de TOBs (Top of Book) y utilizar esos datos para calcular y cotizar contratos de futuros
basados en el activo subyacente.

Su nombre se debe ha que implemente algo similar a lo que hice para los providers. Hay un trait DataConsumer para que en
caso de querer agregar nuevos consumer sea posible a traves de este trait. Esto hace que sea mas escalable.

Hay implementado dos tipos de consumer, uno es el `MarketMaker` que usa la data para calcular el precio de los contratos
de futuros. El otro consumer, denominado `WsConsumer`, que usa los datos generados por los providers para propagarlos
por web socket a todos los que se conecte.

## Como testear

Lo primero que hay que hacer es correr la aplicación con el comando

```shell
cargo run
```
Una vez que este corriendo se podran ver los datos generados de los precios por pantalla. Tambien se podra ver los precios
de los contratos de futuros.

En caso de querer probar por web socket, una manera rapido de hacerlo es con wscat.
Para instalar wscat correr el siguiente comando:

```shell
npm install -g wscat
```

Una vez instalado se podra conectar a la aplicacion por web socket con el siguiente comando:

```shell
wscat -c ws://127.0.0.1:8080
```


