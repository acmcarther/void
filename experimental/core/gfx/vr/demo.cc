#include <stdio.h>

#include "third_party/openvr/headers/openvr.h"

int main(int argc, char **argv) {
  printf("Hello World\n");

  vr::HmdError err;
  vr::IVRSystem *sys =
      vr::VR_Init(&err, vr::EVRApplicationType::VRApplication_Utility);

  if (sys == nullptr) {
    printf("Failed to init system with some issue, %i", err);
  }

  vr::VR_Shutdown();

  return 0;
}
