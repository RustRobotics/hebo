include ./config.mk

.PHONY: all check test ptest clean

all :

check : test

test :
	$(MAKE) -C broker test

ptest :
	$(MAKE) -C broker ptest

reallyclean : clean
clean : 
	$(MAKE) -C broker clean
