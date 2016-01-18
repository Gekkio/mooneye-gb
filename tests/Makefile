# This file is part of Mooneye GB.
# Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
#
# Mooneye GB is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# Mooneye GB is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
WLA ?= wla-gb
WLALINK ?= wlalink

LATEX ?= pdflatex
IMCONVERT ?= convert

BUILD_PATH := build

SRC := $(filter-out ./common/%.s,$(shell find . -type f -name '*.s'))

LATEX_SRC := $(shell find . -type f -name '*.tex')

all: $(addprefix $(BUILD_PATH)/, $(patsubst %.s,%.gb, $(SRC)))

$(BUILD_PATH)/%.o: %.s common/*.s
	@mkdir -p $(BUILD_PATH)/$(dir $<)
	@cd $(dir $<) && $(WLA) -o $(WLAFLAGS) $(notdir $<) $(abspath $@)

$(BUILD_PATH)/%.link: $(BUILD_PATH)/%.o
	@mkdir -p $(dir $<)
	@echo "[objects]\n$(notdir $<)" > $@

$(BUILD_PATH)/%.gb: $(BUILD_PATH)/%.link
	@mkdir -p $(dir $<)
	@cd $(dir $<) && $(WLALINK) -S $(notdir $<) $(abspath $@)
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
.PHONY: clean all
