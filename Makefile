.PHONY: all
all: favicon schema queries

.PHONY: favicon
favicon: logo/favicon.ico

logo/favicon.ico: logo/logo.svg
	@echo "Generating favicon..."
	convert $< -background none -define icon:auto-resize=256,128,96,64,48,32,16 $@

.PHONY: schema
schema: gql/schema.graphql

gql/schema.graphql: gql/schema/*.graphql
	@echo "Generating GraphQL schema..."
	touch $@

	echo "type Query {" >> $@
	cat gql/schema/* | sed -n '/^extend type Query/{ n; p }' >> $@
	echo "}" >> $@
	echo "" >> $@

	echo "type Mutation {" >> $@
	cat gql/schema/* | sed -n '/^extend type Mutation/{ n; p }' >> $@
	echo "}" >> $@
	echo "" >> $@

	echo "type Subscription {" >> $@
	cat gql/schema/* | sed -n '/^extend type Subscription/{ n; p }' >> $@
	echo "}" >> $@
	echo "" >> $@

	cat $^ | sed -e '/\(^extend type\|^type Query\|^type Mutation\|^type Subscription\)/,+3d' >> $@

.PHONY: queries
queries: $(patsubst gql/query/%.graphql,gql/query/%.rs,$(wildcard gql/query/*.graphql))

gql/query/%.rs: gql/query/%.graphql
	@echo "Generating GraphQL query..."
	graphql-client generate --schema-path gql/schema.graphql $<

.PHONY: clean
clean:
	rm -f gql/schema.graphql
	rm -f logo/favicon.ico
	rm -f gql/query/*.rs
