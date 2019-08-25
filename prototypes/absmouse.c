#include <linux/uinput.h>
#include <fcntl.h>

// adapted from example at https://www.kernel.org/doc/html/v4.12/input/uinput.html

void emit(int fd, int type, int code, int val)
{
   struct input_event ie;

   ie.type = type;
   ie.code = code;
   ie.value = val;
   /* timestamp values below are ignored */
   ie.time.tv_sec = 0;
   ie.time.tv_usec = 0;

   write(fd, &ie, sizeof(ie));
}

int main(void)
{
   struct uinput_setup usetup;
   struct uinput_abs_setup abs_x_setup;
   struct uinput_abs_setup abs_y_setup;
   int i = 50;

   int fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);

   /* enable mouse button left and absolute events */
   ioctl(fd, UI_SET_EVBIT, EV_KEY);
   ioctl(fd, UI_SET_KEYBIT, BTN_LEFT);

   ioctl(fd, UI_SET_EVBIT, EV_ABS);
   ioctl(fd, UI_SET_ABSBIT, ABS_X);
   ioctl(fd, UI_SET_ABSBIT, ABS_Y);
   memset(&abs_x_setup, 0, sizeof(abs_x_setup));
   memset(&abs_y_setup, 0, sizeof(abs_y_setup));
    abs_x_setup.code = ABS_X;
    abs_x_setup.absinfo.minimum = 0;
    abs_x_setup.absinfo.maximum = 1920;

    abs_y_setup.code = ABS_Y;
    abs_y_setup.absinfo.minimum = 0;
    abs_y_setup.absinfo.maximum = 1080;
    ioctl(fd, UI_ABS_SETUP, &abs_x_setup);
    ioctl(fd, UI_ABS_SETUP, &abs_y_setup);


   memset(&usetup, 0, sizeof(usetup));
   usetup.id.bustype = BUS_USB;
   usetup.id.vendor = 0x1234; /* sample vendor */
   usetup.id.product = 0x5678; /* sample product */
   strcpy(usetup.name, "Example device");

   ioctl(fd, UI_DEV_SETUP, &usetup);
   ioctl(fd, UI_DEV_CREATE);

   char sysfs_device_name[16];
    ioctl(fd, UI_GET_SYSNAME(sizeof(sysfs_device_name)), sysfs_device_name);
    printf("%s\n", sysfs_device_name);

   /*
    * On UI_DEV_CREATE the kernel will create the device node for this
    * device. We are inserting a pause here so that userspace has time
    * to detect, initialize the new device, and can start listening to
    * the event, otherwise it will not notice the event we are about
    * to send. This pause is only needed in our example code!
    */
   sleep(1);
//    sleep(30);

   /* Move the mouse diagonally, 5 units per axis */
   while (i--) {
      emit(fd, EV_ABS, ABS_X, 1000 + (i * 3));
      emit(fd, EV_ABS, ABS_Y, 500 + (i * 3));
      emit(fd, EV_SYN, SYN_REPORT, 0);
      usleep(15000);
   }

   /*
    * Give userspace some time to read the events before we destroy the
    * device with UI_DEV_DESTOY.
    */
   sleep(1);

   ioctl(fd, UI_DEV_DESTROY);
   close(fd);

   return 0;
}