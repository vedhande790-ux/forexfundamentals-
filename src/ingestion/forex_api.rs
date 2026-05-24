use crate::model::

    tick::

        Tick;


pub async fn fetch_forex_prices()

-> Vec<Tick>

{

    vec![

        Tick {

            symbol:

                "EURUSD"

                    .to_string(),

            price:

                1.10,

            timestamp:

                123456,
        },


        Tick {

            symbol:

                "GBPUSD"

                    .to_string(),

            price:

                1.25,

            timestamp:

                123457,
        }
    ]
}