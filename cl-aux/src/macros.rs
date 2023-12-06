macro_rules! _check_capacity {
  ($elem:expr) => {{
    let capacity_upper_bound = crate::CapacityUpperBound::capacity_upper_bound($elem);
    let length = crate::Length::length($elem);
    if length >= capacity_upper_bound {
      return Err(crate::Error::InsufficientCapacity(capacity_upper_bound));
    }
  }};
}

macro_rules! _check_indcs {
  ($elem:expr, $( $idx:expr ),*) => {{
    let length = crate::Length::length($elem);
    if $( $idx >= length || )* false {
      return Err(crate::Error::OutOfBounds(length));
    }
  }};
}

macro_rules! _get {
  ($elem:expr, $idx:expr) => {{
    $elem.get($idx).ok_or(crate::Error::OutOfBounds($idx))
  }};
}

macro_rules! _get_mut {
  ($elem:expr, $idx:expr) => {{
    $elem.get_mut($idx).ok_or(crate::Error::OutOfBounds($idx))
  }};
}
