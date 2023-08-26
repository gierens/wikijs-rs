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
