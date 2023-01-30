use esp_idf_sys::*;

#[inline]
#[allow(unused)]
pub fn ble_npl_hw_enter_critical() {
  unsafe {
    _ = npl_freertos_hw_enter_critical();
  }
}

#[inline]
#[allow(unused)]
pub fn ble_npl_hw_exit_critical() {
  unsafe {
    _ = npl_freertos_hw_exit_critical(0);
  }
}
