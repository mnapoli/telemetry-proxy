deploy:
	git pull
	cargo build --release
	sudo rm -f /etc/supervisor/conf.d/telemetry-proxy.conf
	sudo cp supervisor/telemetry-proxy.conf /etc/supervisor/conf.d/telemetry-proxy.conf
	sudo /etc/init.d/supervisor restart
	wait 1
	sudo supervisorctl status
