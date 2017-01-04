# Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

WLA ?= wla-gb
WLALINK ?= wlalink

LATEX ?= pdflatex
IMCONVERT ?= convert

BUILD_PATH := build

SRC := $(filter-out ./common/%.s,$(shell find . -type f -name '*.s'))
OBJS := $(addprefix $(BUILD_PATH)/, $(TARGETS:.s=.o))

LATEX_SRC := $(shell find . -type f -name '*.tex')

all: $(addprefix $(BUILD_PATH)/, $(patsubst %.s,%.gb, $(SRC)))

$(BUILD_PATH):
	@mkdir $(BUILD_PATH)

$(OBJS): | $(BUILD_PATH)

$(BUILD_PATH)/flags: force | $(BUILD_PATH)
	@echo '${WLAFLAGS}' | cmp -s - $@ || echo '${WLAFLAGS}' > $@

$(BUILD_PATH)/%.o: %.s common/*.s $(BUILD_PATH)/flags
	@mkdir -p $(dir $@)
	@$(WLA) -I $(abspath common) $(WLAFLAGS) -o $(abspath $@) $<

$(BUILD_PATH)/%.link: $(BUILD_PATH)/%.o
	@printf "[objects]\n%s" $< > $@

$(BUILD_PATH)/%.gb: $(BUILD_PATH)/%.link
	@$(WLALINK) -S $< $(abspath $@)
	@echo --- $(notdir $(basename $@))

$(BUILD_PATH)/%.pdf: %.tex
	@mkdir -p $(BUILD_PATH)/$(dir $<)
	@$(LATEX) -halt-on-error -interaction=batchmode -output-directory=$(dir $@) $<

$(BUILD_PATH)/%.png: $(BUILD_PATH)/%.pdf
	@$(IMCONVERT) -density 150 $< -flatten $@

allpdf: $(addprefix $(BUILD_PATH)/, $(patsubst %.tex,%.pdf, $(LATEX_SRC)))

allpng: $(addprefix $(BUILD_PATH)/, $(patsubst %.tex,%.png, $(LATEX_SRC)))

clean:
	@rm -rf $(BUILD_PATH)

.PRECIOUS: $(BUILD_PATH)/%.pdf
.PHONY: clean all force
