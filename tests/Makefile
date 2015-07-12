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
	@cd $(dir $<) && $(WLALINK) $(notdir $<) $(abspath $@)
	@echo --- $(notdir $(basename $@))

clean:
	@rm -rf $(BUILD_PATH)

.SECONDARY:
.PHONY: clean all
