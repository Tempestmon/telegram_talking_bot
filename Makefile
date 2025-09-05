TENANT = tempestmon
VERSION = 0.0.1

build:
	docker build -t $(TENANT)/mopsorez_bot:v$(VERSION) -f Dockerfile .

tag:
	docker tag $(TENANT)/mopsorez_bot:$(VERSION) $(TENANT)/mopsorez_bot:latest

push: 
	docker push $(TENANT)/mopsorez_bot:$(VERSION) && docker push $(TENANT)/mopsorez_bot:latest

run:
	docker run --rm -it $(TENANT)/mopsorez_bot:$(VERSION)
