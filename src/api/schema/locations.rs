use crate::api::Context;
use ensure_config_rust_client::{
    apis::locations_api::{self, GetAllLocationsParams, GetLocationByIdParams},
    models::Location,
};
use juniper::{graphql_object, FieldResult};
use tokio_compat_02::FutureExt;
pub struct LocationNode(Location);

impl From<Location> for LocationNode {
    fn from(location: Location) -> Self {
        Self(location)
    }
}

#[graphql_object(context = Context)]
#[graphql(name = "Location", description = "A location or place in a community")]
impl LocationNode {
    pub async fn id(&self) -> Option<String> {
        self.0._id.clone()
    }
    pub async fn name(&self) -> String {
        self.0.name.clone()
    }
    pub async fn description(&self) -> Option<String> {
        self.0.description.clone()
    }
    pub async fn parent_location(&self, ctx: &Context) -> FieldResult<Option<LocationNode>> {
        match &self.0.parent_location {
            Some(id) => location_node(ctx, id.clone()).await,
            None => Ok(None),
        }
    }
    pub async fn location_type(&self) -> Option<String> {
        self.0.location_type.clone()
    }
    pub async fn motion_profile(&self) -> Option<String> {
        self.0.motion_profile.clone()
    }
    pub async fn geofence_profile(&self) -> Option<String> {
        self.0.geofence_profile.clone()
    }
    pub async fn residents(&self) -> Option<Vec<String>> {
        self.0.residents.clone()
    }
    pub async fn devices(&self) -> Option<Vec<String>> {
        self.0.devices.clone()
    }
    pub async fn beacons(&self) -> Option<Vec<String>> {
        self.0.beacons.clone()
    }
    pub async fn subjects(&self, ctx: &Context) -> FieldResult<Vec<LocationNode>> {
        match &self.0.subjects {
            Some(subject_ids) => {
                let mut subject_nodes = vec![];
                for subject_id in subject_ids.iter() {
                    if let Some(subject_node) = location_node(ctx, subject_id.clone()).await? {
                        subject_nodes.push(subject_node)
                    }
                }
                Ok(subject_nodes)
            }
            None => Ok(vec![]),
        }
    }
    pub async fn children(&self, ctx: &Context) -> FieldResult<Vec<LocationNode>> {
        match &self.0._id {
            Some(id) => children_location_nodes(ctx, id.clone()).await,
            None => Ok(vec![]),
        }
    }
    pub async fn rounds(&self) -> Option<Vec<String>> {
        self.0.rounds.clone()
    }
    pub async fn created_at(&self) -> Option<String> {
        self.0.created_at.clone()
    }
    pub async fn updated_at(&self) -> Option<String> {
        self.0.updated_at.clone()
    }
    pub async fn revision(&self) -> Option<i32> {
        self.0.revision.map(|r| r as i32)
    }
}

pub async fn location_nodes(ctx: &Context) -> FieldResult<Vec<LocationNode>> {
    let params = GetAllLocationsParams {
        name: None,
        devices: None,
        residents: None,
        motion_profile: None,
        geofence_profile: None,
        parent_location: None,
        subjects: None,
        location_type: None,
    };
    filter_location_nodes(ctx, params).await
}

pub async fn location_node(ctx: &Context, id: String) -> FieldResult<Option<LocationNode>> {
    let params = GetLocationByIdParams { id };
    let response = locations_api::get_location_by_id(&ctx.api_config(), params)
        .compat()
        .await?;
    match response.status {
        http::status::StatusCode::OK => match response.entity.unwrap() {
            locations_api::GetLocationByIdSuccess::Status200(location) => Ok(Some(location.into())),
            s => Err(format!("{:?}", s).into()),
        },
        s => Err(format!("{}", s).into()),
    }
}

async fn children_location_nodes(ctx: &Context, id: String) -> FieldResult<Vec<LocationNode>> {
    let params = GetAllLocationsParams {
        name: None,
        devices: None,
        residents: None,
        motion_profile: None,
        geofence_profile: None,
        parent_location: Some(id),
        subjects: None,
        location_type: None,
    };
    filter_location_nodes(ctx, params).await
}

async fn filter_location_nodes(
    ctx: &Context,
    params: GetAllLocationsParams,
) -> FieldResult<Vec<LocationNode>> {
    let response = locations_api::get_all_locations(&ctx.api_config(), params)
        .compat()
        .await?;
    match response.status {
        http::status::StatusCode::NO_CONTENT => Ok(vec![]),
        http::status::StatusCode::OK => match response.entity.unwrap() {
            locations_api::GetAllLocationsSuccess::Status200(locations) => {
                Ok(locations.into_iter().map(LocationNode::from).collect())
            }
            s => Err(format!("{:?}", s).into()),
        },
        s => Err(format!("{}", s).into()),
    }
}
