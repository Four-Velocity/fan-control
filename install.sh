cargo build --release

cp ./target/release/fan-control /usr/bin/fan-control

cp systemd/* /lib/systemd/system/

systemctl enable fan-control.service
systemctl start fan-control.service

systemctl enable fan-control.timer
systemctl start fan-control.timer
