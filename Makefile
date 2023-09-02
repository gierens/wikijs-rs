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

.PHONY: clean
clean:
	rm -f gql/schema.graphql
	rm -f logo/favicon.ico
