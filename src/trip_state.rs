use crate::trip_event::TripEvent;

pub(crate) struct TripState {
    events: Vec<TripEvent>,
}

impl TripState {
    pub(crate) fn new() -> Self {
        TripState { events: vec![] }
    }

    pub(crate) fn record(&mut self, event: TripEvent) {
        self.events.push(event);
    }

    pub(crate) fn events(&self) -> &Vec<TripEvent> {
        &self.events
    }
}
