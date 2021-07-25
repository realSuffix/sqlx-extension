use crate::models::field::Field;

#[derive(Default)]
pub struct QueryBuilder<'a> {
    table_name: Option<&'a str>,
    entity_fields: Option<&'a [Field]>,
}

impl<'a> QueryBuilder<'a> {
    /// This function creates an empty query builder.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// This method sets the table name for the query builder.
    pub fn table_name(mut self, name: &'a str) -> Self {
        self.table_name = Some(name);
        self
    }

    /// This method sets the entity attributes for the query builder
    pub fn entity_attributes(mut self, entity_fields: &'a [Field]) -> Self {
        self.entity_fields = Some(entity_fields);
        self
    }

    /// This method builds an insert statement.
    /// If the returns array is non-empty, those paramters will be added to the
    /// `RETURNING` clause.
    pub fn build_insert(&self, returns: &[Field]) -> Option<String> {
        let names = Self::fields_to_names(&self.entity_fields?);

        let returns = if !returns.is_empty() {
            let return_names = Self::fields_to_names(&returns);
            format!("RETURNING {}", return_names)
        } else {
            "".into()
        };

        Some(format!(
            "INSERT INTO {} ({}) VALUES ({}) {};",
            self.table_name?,
            names,
            Self::get_n_placeholders(self.entity_fields?.len()),
            returns,
        ))
    }

    /// This method builds a simply retrieve query without a `WHERE` clause.
    pub fn build_retrieve(&self) -> Option<String> {
        Some(format!(
            "SELECT * FROM {};",
            self.table_name?,
        ))
    }

    /// This method builds a retrieve query with a `WHERE` clause which `AND`s all
    /// passed attributes.
    pub fn build_retrieve_where(&self, where_fields: &[Field]) -> Option<String> {
        Some(format!(
            "SELECT * FROM {} {};",
            self.table_name?,
            Self::where_and_fields(where_fields)
        ))
    }

    /// This method builds a delete query with a `WHERE` clause that ands all passed attributes.
    pub fn build_delete_where(&self, where_fields: &[Field]) -> Option<String> {
        Some(format!(
            "DELETE FROM {} {};",
            self.table_name?,
            Self::where_and_fields(where_fields)
        ))
    }

    /// This method builds an update query. The first array contains all fields which ought to
    /// be set, the second one is used to construct a `WHERE` clause.
    pub fn build_update(&self, set: &[Field], where_fields: &[Field]) -> Option<String> {
        // Make sure that the parameters to be set aren't empty
        if set.is_empty() {
            return None;
        }

        // generates the "SET par = $1" part
        let set_par = Self::generate_equals_field(set.iter())
            .collect::<Vec<_>>()
            .join(", ");

        // generates the filter part
        let filter = Self::where_and_fields(where_fields);

        Some(format!(
            "UPDATE {} SET {} {};",
            self.table_name?,
            set_par,
            filter
        ))
    }

    /// This utility method generates a where clause in case the
    /// passed array is non-empty.
    fn where_and_fields(where_fields: &[Field]) -> String {
        if where_fields.is_empty() {
            "".into()
        } else {
            let s = Self::generate_equals_field(where_fields.iter())
                .collect::<Vec<_>>()
                .join(" AND ");
            format!("WHERE {}", s)
        }
    }

    /// This method converts the incoming interator and maps each item (=field) to the following:
    /// `{FIELD} = ${INDEX}`;
    fn generate_equals_field<'b>(input: impl Iterator<Item = &'b Field> + 'b) -> impl Iterator<Item = String> + 'b {
        input
            .enumerate()
            .map(|(index, f)| format!("{} = ${}", f.name(), index+1))
    }

    /// This function maps an array of fields to their respective names.
    fn fields_to_names(fields: &[Field]) -> String {
        fields.iter().map(Field::name).collect::<Vec<_>>().join(", ")
    }

    /// This function generates n placeholders ala "$1, $2" etc.
    fn get_n_placeholders(n: usize) -> String {
        (1..n + 1)
            .into_iter()
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
