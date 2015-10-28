# This file is part of Mooneye GB.
# Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
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

BUILD_PATH := build

SRC := $(filter-out ./common/%.s,$(shell find -type f -name '*.s'))

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

clean:
	@rm -rf $(BUILD_PATH)

.PHONY: clean all
