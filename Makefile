TENANT = tempestmon
VERSION = v0.0.1

build:
	docker build -t $(TENANT)/mopsorez_telegram_bot:$(VERSION) -f Dockerfile .

tag:
	docker tag $(TENANT)/mopsorez_telegram_bot:$(VERSION) $(TENANT)/mopsorez_telegram_bot:latest

push: 
	docker push $(TENANT)/mopsorez_telegram_bot:$(VERSION) && docker push $(TENANT)/mopsorez_telegram_bot:latest

run:
	docker run --rm -it $(TENANT)/mopsorez_telegram_bot:$(VERSION)
