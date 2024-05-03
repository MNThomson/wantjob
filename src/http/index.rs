use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use leptos::{view, CollectView};
use serde::Deserialize;

use super::{
    components::{htmlify, JobPostingDetails, JobSummary, Layout},
    AppState,
};
use crate::http::components::HomePageDetails;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", get(|| async { Redirect::temporary("/jobs") }))
        .route("/jobs", get(job_handler))
        .route("/htmx/jobs/details/:id", get(htmx_jobs_details));
}

#[derive(Clone)]
pub struct JobDetails {
    pub id: String,
    pub title: String,
    pub company: String,
    pub location: String,
    pub salary: String,
    pub description: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JobQueryParams {
    current_job_id: Option<String>,
}

async fn job_handler(
    State(_state): State<AppState>,
    query_params: Query<JobQueryParams>,
) -> impl IntoResponse {
    let jobs = mock_jobs(30);

    let job = query_params.current_job_id.is_some().then(|| {
        get_mock_job(
            query_params
                .current_job_id
                .clone()
                .expect("the current job id to be some"),
        )
    });

    let h = htmlify(|| {
        return view! {
            <Layout>
                <div class="h-full flex">
                    <div class="basis-[48rem] mr-2 overflow-auto h-full">
                        {jobs.into_iter().map(|n| view! {
                            <JobSummary job=n />
                        }).collect_view()}
                    </div>
                    <div class="basis-full bg-dark-weak rounded-sm overflow-auto h-full">
                        {if job.is_some() {
                            view! {<JobPostingDetails job=job.unwrap() />}
                        } else {view! {
                            <HomePageDetails/>
                        }}}
                    </div>
                </div>
            </Layout>
        };
    });

    return (StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], h);
}

async fn htmx_jobs_details(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let job = get_mock_job(id);

    let h = htmlify(|| {
        return view! {
            <JobPostingDetails job=job />
        };
    });

    return (StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], h);
}
/* --------------------------------------------------------------------------------------- */

fn mock_jobs(num: usize) -> Vec<JobDetails> {
    let jobs: Vec<JobDetails> = vec![
        JobDetails {
            id: "2".to_string(),
            title: "Software Developer Intern".to_string(),
            company: "Cloudflare".to_string(),
            location: "New York".to_string(),
            salary: "120K USD".to_string(),
            description: "You're gonna be the coffee boy, sorry".to_string(),
        },
        JobDetails {
            id: "1".to_string(),
            title: "Sr Rust Developer".to_string(),
            company: "GitHub".to_string(),
            location: "Vancouver".to_string(),
            salary: "187K CAD".to_string(),
            description: "Join the up and coming hype driven development!".to_string(),
        },
        JobDetails {
            id: "3".to_string(),
            title: "Technical Product Manager - AI Enhancement Policy Team".to_string(),
            company: "Amazon".to_string(),
            location: "Seattle".to_string(),
            salary: "278K USD".to_string(),
            description: "You are the new team SCRUMLORD and AGILEMASTER".to_string(),
        },
        JobDetails {
            id: "alsdkjaslkdsf".to_string(),
            title: "Jr Fullstack Developer".to_string(),
            company: "Adobe".to_string(),
            location: "Seattle".to_string(),
            salary: "73K CAD".to_string(),
            description: "Have fun you reach andy".to_string(),
        },
        JobDetails {
            id: "ldsfjosadjasd".to_string(),
            title: "Staff QA Specialist".to_string(),
            company: "Google".to_string(),
            location: "San Fransisco".to_string(),
            salary: "572K USD".to_string(),
            description: "You are head honcho and hate your job".to_string(),
        },
        JobDetails {
            id: "lidfsjasdlkajsd".to_string(),
            title: "Software Developer".to_string(),
            company: "Crowdstrike".to_string(),
            location: "New York".to_string(),
            salary: "147K USD".to_string(),
            description: "Want to investigate Snarling Bear or Smiling Panda?".to_string(),
        },
    ]
    .into_iter()
    .cycle()
    .take(num)
    .collect();

    return jobs;
}

fn get_mock_job(id: String) -> JobDetails {
    return mock_jobs(6)
        .iter()
        .find(|j| return j.id.eq(&id))
        .unwrap()
        .clone();
}
