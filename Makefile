DESTDIR=
BINDIR=/usr/bin
SHAREDIR=/usr/share/ct
ETCDIR=/etc/ct
SYSTEMDDIR=/etc/systemd/system

install:
	install -d -m 0755 -o root -g root $(DESTDIR)/$(BINDIR)
	install    -m 0755 -o root -g root ct $(DESTDIR)/$(BINDIR)
	install -d -m 0755 -o root -g root $(DESTDIR)/$(SHAREDIR)
	install    -m 0755 -o root -g root ct-daemon-functions.sh $(DESTDIR)/$(SHAREDIR)
	install -d -m 0755 -o root -g root $(DESTDIR)/$(ETCDIR)
	install    -m 0755 -o root -g root ct.config $(DESTDIR)/$(ETCDIR)
	install -d -m 0755 -o root -g root $(DESTDIR)/$(SYSTEMDDIR)
	install    -m 0755 -o root -g root ct.service $(DESTDIR)/$(SYSTEMDDIR)
uninstall:
	-rm $(DESTDIR)/$(BINDIR)/ct
	-rm -rf $(DESTDIR)/$(SHAREDIR)
	-rm -rf $(DESTDIR)/$(ETCDIR)
	-rm $(DESTDIR)/$(SYSTEMDDIR)/ct.service
