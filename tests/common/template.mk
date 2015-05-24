WLA := wla-gb
WLALINK := wlalink

BASE_PATH := $(CURDIR)/..
BUILD_PATH := $(BASE_PATH)/build/$(notdir $(basename $(CURDIR)))

all: $(addprefix $(BUILD_PATH)/, $(addsuffix .gb, $(basename $(wildcard *.s))))

$(BUILD_PATH)/%.o: %.s ../common/common.i
	@mkdir -p $(BUILD_PATH)
	@$(WLA) -o $(WLAFLAGS) $< $@

$(BUILD_PATH)/%.link: $(BUILD_PATH)/%.o
	@mkdir -p $(BUILD_PATH)
	@echo "[objects]\n$<" > $@

$(BUILD_PATH)/%.gb: $(BUILD_PATH)/%.link
	@mkdir -p $(BUILD_PATH)
	@$(WLALINK) $< $@
	@echo --- $(notdir $(basename $@))

clean:
	@rm -fr $(BUILD_PATH)

.SECONDARY:
.PHONY: clean all
