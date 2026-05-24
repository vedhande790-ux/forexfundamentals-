use std::sync::{

    Arc,

    RwLock,
};

use tokio::time::{

    sleep,

    Duration,
};

use crate::

    engine::

        market_engine::

            MarketEngine;


use crate::ingestion::forex_api::fetch_forex_prices;

pub async fn start_runtime(engine: Arc<RwLock<MarketEngine>>) {
    loop {
        let ticks = fetch_forex_prices().await;

        {
            let mut engine = engine.write().unwrap();


            for tick in ticks {

                engine

                    .process_tick(

                        tick
                    );
            }
        }


        sleep(

            Duration::

                from_secs(

                    120
                )

        )

        .await;
    }
}